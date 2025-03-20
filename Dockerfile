FROM rust:1.83-slim-bullseye AS builder

RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY ./src ./src

RUN cargo build --release && strip /app/target/release/axum-surrealdb-boilerplate

FROM gcr.io/distroless/cc AS runner

WORKDIR /app

COPY --from=builder /app/target/release/axum-surrealdb-boilerplate .

CMD ["/app/axum-boilerplate-surrealdb"]
