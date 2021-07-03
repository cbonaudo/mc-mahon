FROM rust:1.53 as builder

RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /usr/src/mcmahon

COPY . ./
RUN cargo build --release
COPY ./env.toml ./target/release/

FROM debian:buster-slim
RUN apt-get update && apt-get -f install && apt-get install -y libcurl4-openssl-dev && apt-get -y install curl

COPY --from=builder /usr/src/mcmahon/target/release/ ./

USER 1000
CMD ["./mcmahon"]
