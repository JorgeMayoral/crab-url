FROM lukemathwalker/cargo-chef:latest-rust-1.72.0 as chef
WORKDIR /app
RUN apt update && apt install lld clang -y

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin crab-url

FROM node:18-alpine AS frontend
WORKDIR /app
COPY frontend/package.json .
RUN npm install
COPY frontend .
RUN npm run build

FROM ubuntu:latest AS runtime
WORKDIR /app
RUN apt-get update -y \
  && apt-get install -y --no-install-recommends openssl ca-certificates libcurl4-openssl-dev libsqlite3-dev \
  && apt-get autoremove -y \
  && apt-get clean -y \
  && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/crab-url crab-url
COPY --from=frontend /app/dist ./frontend/dist
ENV LOGGER json
ENV LOG_DIRECTIVES crab_url=info
ENTRYPOINT ["./crab-url"]
