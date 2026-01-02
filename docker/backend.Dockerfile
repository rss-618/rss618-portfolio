FROM rust:1.92-alpine AS builder

RUN apk add --no-cache musl-dev

WORKDIR /app

COPY proto/gen/rust ./proto/gen/rust
COPY backend/Cargo.toml backend/Cargo.lock* ./backend/
RUN mkdir backend/src && echo "fn main() {}" > backend/src/main.rs
RUN cd backend && cargo build --release
RUN rm -rf backend/src

COPY backend/src ./backend/src
RUN touch backend/src/main.rs
RUN cd backend && cargo build --release

FROM alpine:3.21 AS runner

RUN apk add --no-cache ca-certificates

WORKDIR /app

COPY --from=builder /app/backend/target/release/backend ./backend

ENV HOST=0.0.0.0
ENV PORT=3000

EXPOSE 3000

CMD ["./backend"]
