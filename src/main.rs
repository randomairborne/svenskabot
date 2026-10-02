#![allow(clippy::result_large_err)]
use poise::serenity_prelude::CreateEmbed;
use poise::{CreateReply, serenity_prelude as serenity};
use std::collections::HashMap;
use std::fmt::Write;

struct Data {
    tags: HashMap<String, String>,
} // User data, which is stored and accessible in all command invocations

type Context<'a> = poise::Context<'a, Data, poise::serenity_prelude::Error>;

/// Gets a link to a FAQ entry
#[poise::command(slash_command)]
async fn faq(
    ctx: Context<'_>,
    #[description = "FAQ name"]
    #[autocomplete = "name_autocomplete"]
    name: String,
) -> Result<(), poise::serenity_prelude::Error> {
    let response = ctx
        .data()
        .tags
        .get(&name)
        .map_or("That FAQ item doesn't exist.", |v| v);
    let embed = CreateEmbed::new().description(response);
    ctx.send(CreateReply::new().embed(embed)).await?;
    Ok(())
}

async fn name_autocomplete(
    ctx: Context<'_>,
    partial: &str,
) -> serenity::CreateAutocompleteResponse {
    let mut choices: Vec<_> = ctx
        .data()
        .tags
        .keys()
        .filter(|v| v.contains(partial))
        .take(25)
        .collect();
    choices.sort();

    let choices = choices
        .into_iter()
        .map(|n| serenity::AutocompleteChoice::new(n.clone(), n.clone()))
        .collect();
    serenity::CreateAutocompleteResponse::new().set_choices(choices)
}

/// Gets a list of FAQ links
#[poise::command(slash_command)]
async fn faqs(ctx: Context<'_>) -> Result<(), poise::serenity_prelude::Error> {
    let mut response = String::with_capacity(2000);
    for (key, _value) in ctx.data().tags.iter() {
        let _ = writeln!(response, "- `{key}`");
    }
    let embed = CreateEmbed::new().description(response);
    ctx.send(CreateReply::new().embed(embed).ephemeral(true))
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let tags = {
        let config_path = std::env::args_os()
            .nth(1)
            .expect("1 argument required, path to config");
        let config_handle = std::fs::OpenOptions::new()
            .read(true)
            .open(config_path)
            .expect("failed to open config");
        serde_json::from_reader(config_handle).expect("Invalid JSON in config")
    };
    let token = std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN");
    let intents = serenity::GatewayIntents::empty();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![faq(), faqs()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(
                    ctx,
                    &framework.options().commands,
                )
                .await?;
                Ok(Data { tags })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}
