FROM rust:1.92-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app

COPY backend/Cargo.toml backend/Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

COPY backend/src ./src
RUN touch src/main.rs
RUN cargo build --release

FROM alpine:3.21 AS runner

RUN apk add --no-cache ca-certificates

WORKDIR /app

COPY --from=builder /app/target/release/backend ./backend

ENV HOST=0.0.0.0
ENV PORT=3000

EXPOSE 3000

CMD ["./backend"]
