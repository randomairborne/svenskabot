FROM rust AS builder

COPY . .

RUN cargo build --release

FROM debian:stable-slim

COPY --from=builder target/release/svenskabot /usr/bin/svenskabot
COPY ./config.json /etc/svenskabot/config.json 

CMD ["svenskabot", "/etc/svenskabot/config.json"]
