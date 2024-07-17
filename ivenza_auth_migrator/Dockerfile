FROM rust:latest AS builder

# You'll need to change `libmysqlclient-dev` to `libpq-dev` if you're using Postgres
RUN apt-get update && apt-get install -y default-libmysqlclient-dev

RUN mkdir /source
WORKDIR /source
COPY ./ ./
# added --config net.git-fetch-with-client=tue flag to cargo to work around 137 error (oom), 
# like described here https://github.com/rust-lang/cargo/issues/10781
RUN cargo build --config net.git-fetch-with-cli=true  --release
RUN strip ./target/release/ivenza_auth_migrator

# extract temporary packages needed in runtime to /dpkg
RUN cd /tmp && \
    apt-get update && apt-get download zlib1g-dev libmariadb3 && \
    mkdir /dpkg && \
    for deb in *.deb; do dpkg --extract $deb /dpkg || exit 10; done

# use Fedora latest as the runtime image. 
# Fedora ships with a good set of openssl libraries, and is more up-to-date than debian or ubuntu.
FROM fedora:latest as runtime
COPY --from=builder /source/target/release/ivenza_auth_migrator /
CMD ["./ivenza_auth_migrator"]
