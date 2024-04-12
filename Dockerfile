FROM rust:1.77.0

WORKDIR /app

COPY . .

ENV SQLX_OFFLINE true

RUN cargo build --release

ENTRYPOINT ["./target/release/eureka"]
