FROM alpine:3.22.1@sha256:4bcff63911fcb4448bd4fdacec207030997caf25e9bea4045fa6c8c44de311d1

RUN apk add --no-cache \
    git=2.49.1-r0

COPY ./target/x86_64-unknown-linux-musl/release/is_affected /usr/local/bin/

WORKDIR /workspace

ENTRYPOINT ["is_affected"]
