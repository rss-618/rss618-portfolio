FROM oven/bun:1 AS builder

WORKDIR /app

COPY frontend/package.json frontend/bun.lock* ./
RUN bun install --frozen-lockfile

COPY frontend/ ./
COPY proto/gen/ts ./src/lib/proto
RUN bun run build

FROM oven/bun:1 AS runner

WORKDIR /app

COPY --from=builder /app/build ./build
COPY --from=builder /app/package.json ./
RUN bun install --production --frozen-lockfile

ENV NODE_ENV=production
ENV PORT=5173

EXPOSE 5173

CMD ["bun", "run", "build"]
