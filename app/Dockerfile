FROM rust:latest

WORKDIR /app

COPY . .

RUN cargo build --release && cargo run

CMD ["./target/release/app"]