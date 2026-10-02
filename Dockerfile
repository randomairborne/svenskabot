FROM rust AS builder

COPY . .

RUN cargo build --release

FROM debian:slim

COPY --from=builder target/release/svenskabot /usr/bin/svenskabot
COPY tags.json /etc/svenskabot/tags.json 

CMD "svenskabot /etc/svenskabot/tags.json"
