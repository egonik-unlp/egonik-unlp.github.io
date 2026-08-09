# ── Build stage ────────────────────────────────────────────────────────────
FROM rust:1.93-slim-bookworm AS builder

RUN apt-get update \
    && apt-get install -y --no-install-recommends pkg-config libssl-dev ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown

# Install a prebuilt cargo-leptos binary instead of compiling it — this avoids
# building OpenSSL from source (openssl-sys), which fails on slim images that
# lack Perl's FindBin.pm.
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall cargo-leptos --locked --no-confirm

WORKDIR /app
COPY . .
RUN cargo leptos build --release

# ── Runtime stage ──────────────────────────────────────────────────────────
FROM debian:bookworm-slim AS runtime

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Server binary + hashed client assets (WASM / JS / CSS) produced by cargo-leptos.
COPY --from=builder /app/target/release/website /app/website
COPY --from=builder /app/target/site /app/site

# Leptos reads these at runtime when no Cargo.toml is present.
ENV LEPTOS_OUTPUT_NAME=website
ENV LEPTOS_SITE_ROOT=site
ENV LEPTOS_SITE_PKG_DIR=pkg
ENV LEPTOS_SITE_ADDR=0.0.0.0:3000
ENV RUST_LOG=info

EXPOSE 3000
CMD ["/app/website"]
