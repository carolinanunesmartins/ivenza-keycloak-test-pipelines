FROM rust:latest AS builder

# You'll need to change `libmysqlclient-dev` to `libpq-dev` if you're using Postgres
RUN apt-get update && apt-get install -y default-libmysqlclient-dev

RUN mkdir /source
WORKDIR /source
COPY ./ ./

RUN cargo build --release
RUN strip ./target/release/ivenza_auth_migrator

# extract temporary packages needed in runtime to /dpkg
RUN cd /tmp && \
    apt-get update && apt-get download zlib1g-dev libmariadb3 && \
    mkdir /dpkg && \
    for deb in *.deb; do dpkg --extract $deb /dpkg || exit 10; done

# use a distroless/cc image as base to run the dynamically linked rust application in
FROM gcr.io/distroless/cc as runtime
COPY --from=builder /dpkg/usr/lib/x86_64-linux-gnu/libmariadb.so.3 ./usr/lib/libmariadb.so.3
COPY --from=builder /dpkg/usr/lib/x86_64-linux-gnu/libz.so /usr/lib/libz.so.1
COPY --from=builder /source/target/release/ivenza_auth_migrator /

CMD ["./ivenza_auth_migrator"]
