FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# Required build argument for sqlx to successfully build project
ARG DATABASE_URL=$DATABASE_URL

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json

ENV SQLX_OFFLINE=true
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release --bin starbet-live

# Certificates
FROM alpine:latest as certs
RUN apk --update add ca-certificates

# We do not need the Rust toolchain to run the binary!
FROM ubuntu AS runtime
WORKDIR /app
COPY --from=certs /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /app/target/release/starbet-live /usr/local/bin
ENTRYPOINT ["/usr/local/bin/starbet-live"]
EXPOSE 6969