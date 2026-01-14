# Stage 1: Build in Linux
FROM rust:alpine AS builder

WORKDIR /app
COPY . .

# Install musl-dev for static linking
RUN apk add --no-cache musl-dev

# Build the binary
RUN cargo build --release

# Stage 2: Runtime
FROM alpine:latest

COPY --from=builder /app/target/release/longest_prefix_match /usr/local/bin/

RUN chmod +x /usr/local/bin/longest_prefix_match

CMD ["tail", "-f", "/dev/null"]