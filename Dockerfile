FROM alpine:3.23.2@sha256:865b95f46d98cf867a156fe4a135ad3fe50d2056aa3f25ed31662dff6da4eb62

RUN apk add --no-cache \
    git=2.49.1-r0

COPY ./target/x86_64-unknown-linux-musl/release/is_affected /usr/local/bin/

WORKDIR /workspace

ENTRYPOINT ["is_affected"]
