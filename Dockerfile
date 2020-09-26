FROM rust:1.46-buster as builder

WORKDIR /app

RUN mkdir src \
    && touch src/lib.rs \
    && apt-get update -y && apt-get install -y libssl-dev ca-certificates

COPY Cargo.toml .

RUN cargo build --release

COPY . .

RUN cargo build --release


FROM debian:buster-slim

RUN apt-get update \
    && apt-get install -y libssl-dev ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/box_hunter .

CMD ["/box_hunter"]
