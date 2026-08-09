# Personal site — Eduardo Gonik

A personal portfolio built as a full-stack **Rust** app: [Leptos](https://leptos.dev)
(SSR + hydration) on an [Axum](https://github.com/tokio-rs/axum) server, built with
[`cargo-leptos`](https://github.com/leptos-rs/cargo-leptos).

## Requirements

- Rust (stable) with the `wasm32-unknown-unknown` target
- `cargo-leptos` (`cargo install cargo-leptos --locked`)

Both are already present on the dev machine.

## Develop

```sh
cargo leptos watch
```

Serves at http://127.0.0.1:3000 with live reload.

## Build & run (release)

```sh
cargo leptos build --release
LEPTOS_SITE_ROOT=target/site ./target/release/website
```

## Docker

```sh
docker build -t eg-site .
docker run --rm -p 3000:3000 eg-site
```

Deployable via Portainer on the home server (or any Docker host). Note: the SSR
server is a running process — it can't be published to GitHub Pages as static files.

The GitHub Pages workflow handles this by rendering the single homepage route during
CI and publishing that HTML with the hydration bundle and static assets. The legacy
JSON resources remain available at `/assets/data/projects.json` and
`/assets/data/works.json`.

## Design

Visual concept is **"Lensing field"** (light bending around mass). See
[`PRODUCT.md`](PRODUCT.md) (strategy/brand) and [`DESIGN.md`](DESIGN.md) (tokens,
type, motion). Key pieces:

- **Fonts** are self-hosted variable woff2 in `public/fonts/` (Bricolage Grotesque /
  Hanken Grotesk / Spline Sans Mono) — no external font requests.
- **The hero canvas** (gravitational lensing) is `public/js/hero-lens.js`;
  scroll-aware nav + reveals are `public/js/interactions.js`. Both are plain, static
  assets loaded with `defer`; nothing runs in WASM.
- All colors are OKLCH in `:root`; every text pair is verified ≥4.5:1 and every
  animation has a `prefers-reduced-motion` fallback.

## Editing content

Almost everything lives in data + small components:

- `src/data/projects.rs` — the project entries (title, tagline, description, tech, links).
- `src/components/hero.rs` — name, headline, intro.
- `src/components/skills.rs` — the skills / tech groups.
- `src/components/research.rs` — publications + the "Research & scientific computing" thread.
- `src/components/contact.rs` — contact links.
- `style/main.scss` — design tokens, type and layout.

## TODO

- Add real **LinkedIn URL** in `src/components/contact.rs` (`LINKEDIN_URL`).
- Add a profile photo / personal images (drop into `public/`, reference from `hero.rs`).
