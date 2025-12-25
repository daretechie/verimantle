# Build stage
FROM rust:1.83-alpine AS builder

WORKDIR /app

RUN apk add --no-cache musl-dev

COPY Cargo.toml Cargo.lock* ./

RUN mkdir src && \
    echo 'fn main() {}' > src/bin/server.rs && \
    echo 'pub fn dummy() {}' > src/lib.rs

RUN cargo build --release --bin arbiter-server && \
    rm -rf src

COPY src ./src

RUN cargo build --release --bin arbiter-server

# Runtime stage
FROM alpine:3.19

RUN apk add --no-cache ca-certificates

WORKDIR /app

COPY --from=builder /app/target/release/arbiter-server /app/arbiter-server

ENV PORT=3003
EXPOSE 3003

CMD ["/app/arbiter-server"]
