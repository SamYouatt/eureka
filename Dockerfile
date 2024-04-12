FROM rust:1.77.0 as builder

WORKDIR /app
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release

FROM rust:1.77.0 as runtime

WORKDIR /app
COPY --from=builder /app/target/release/eureka eureka
COPY configuration configuration
COPY assets assets
COPY node_modules node_modules

ENV APP_ENVIRONMENT production
ENTRYPOINT ["./eureka"]
