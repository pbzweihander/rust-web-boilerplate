FROM clux/muslrust:nightly-2019-05-11

WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src

RUN cargo build --release

FROM alpine:latest

RUN apk --no-cache add tini
ENTRYPOINT ["/sbin/tini", "--"]

COPY --from=0 \
    /app/target/x86_64-unknown-linux-musl/release/rust-web-boilerplate \
    /usr/bin/rust-web-boilerplate

EXPOSE 5000

CMD ["/usr/bin/rust-web-boilerplate", "-p", "5000"]
