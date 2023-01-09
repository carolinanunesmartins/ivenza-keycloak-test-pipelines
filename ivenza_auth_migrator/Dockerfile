FROM rust:latest AS builder

# You'll need to change `libmysqlclient-dev` to `libpq-dev` if you're using Postgres
RUN apt-get update && apt-get install -y curl default-libmysqlclient-dev

RUN mkdir /source
WORKDIR /source
COPY ./Cargo.toml .
COPY ./Cargo.lock .
COPY ./src/ ./src/

RUN cargo build --release
RUN strip ./target/release/ivenza_auth_migrator

FROM debian:buster-slim as runtime
RUN apt-get update && apt-get install -y default-libmysqlclient-dev
COPY --from=builder /source/target/release/ivenza_auth_migrator  /usr/local/bin
CMD ["/usr/local/bin/ivenza_auth_migrator"]