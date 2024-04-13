FROM rust:1.77.0 as builder

WORKDIR /app
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM debian:bookworm-slim as runtime

WORKDIR /app
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/eureka eureka
COPY configuration configuration
COPY assets assets
COPY node_modules node_modules

ENV APP_ENVIRONMENT production
ENTRYPOINT ["./eureka"]
