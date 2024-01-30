FROM rust:1.75 as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update -y && apt-get install -y pkg-config libssl-dev openssl ca-certificates

WORKDIR /fife-bin-calendar

COPY --from=builder /app/target/release/fife-bin-calendar /app/Rocket.toml ./

EXPOSE 8000

CMD ["./fife-bin-calendar"]