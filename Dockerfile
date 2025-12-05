FROM rust:alpine AS builder
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=bind,target=/usr/src/php-dependency-explorer,rw \
    cargo install --path /usr/src/php-dependency-explorer

FROM composer AS php-dependency-explorer
COPY --link --from=builder /usr/local/cargo/bin/php-dependency-explorer /usr/local/bin/php-dependency-explorer
RUN git config --global --add safe.directory /app
ENTRYPOINT ["/sbin/tini", "--", "php-dependency-explorer"]
