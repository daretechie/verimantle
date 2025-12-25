# Build stage
FROM rust:1.83-alpine AS builder

WORKDIR /app

# Install musl-dev for static linking
RUN apk add --no-cache musl-dev

# Copy Cargo files
COPY Cargo.toml Cargo.lock* ./

# Create dummy src to cache dependencies
RUN mkdir src && \
    echo 'fn main() {}' > src/bin/server.rs && \
    echo 'pub fn dummy() {}' > src/lib.rs

RUN cargo build --release --bin gate-server && \
    rm -rf src

# Copy actual source
COPY src ./src

# Build for real
RUN cargo build --release --bin gate-server

# Runtime stage
FROM alpine:3.19

RUN apk add --no-cache ca-certificates

WORKDIR /app

COPY --from=builder /app/target/release/gate-server /app/gate-server

ENV PORT=3001
EXPOSE 3001

CMD ["/app/gate-server"]
