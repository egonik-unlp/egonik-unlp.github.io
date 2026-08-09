# Design

Visual system for Eduardo Gonik's portfolio. Concept: **"Lensing field"** — light
bending around mass. A deep-field (dark) hero resolves into a precise light body and
closes back into the field at contact. One luminous accent = lensed light.

## Theme
Light-primary with two deliberately dark, atmospheric bookends (hero, contact).
Daylight "lab light" body; deep-space hero. Color strategy: **restrained + one
committed luminous accent** (gold), never the SaaS indigo-on-white default.

## Color (OKLCH)
Tokens live in `:root` in `style/main.scss`.

| Role | Token | Value | Notes |
|---|---|---|---|
| Body bg | `--paper` | `oklch(0.983 0.006 265)` | cool near-white, not cream |
| Alt surface | `--surface` | `oklch(0.958 0.008 265)` | skills band |
| Text | `--ink` | `oklch(0.215 0.021 275)` | 16.7:1 on paper |
| Muted text | `--ink-soft` | `oklch(0.435 0.021 273)` | 7.5:1 on paper |
| Hairlines | `--line` / `--line-soft` | `0.905 / 0.935 …` | dividers |
| Dark bg | `--space` / `--space-2` | `0.170 / 0.128 …027 273` | hero, contact |
| On dark | `--on-space` / `--on-space-soft` | `0.962 / 0.735` | 17:1 / 8.2:1 |
| Accent (light) | `--gold` | `oklch(0.815 0.135 79)` | ring, buttons, glow |
| Accent (hot) | `--gold-hot` | `oklch(0.905 0.085 90)` | ring highlight, lit words |
| Accent text | `--gold-deep` | `oklch(0.470 0.105 62)` | links on paper, 6.7:1 |

Accent usage: gold is fills/glows/underlines/arrows; small colored text on paper uses
`--gold-deep` (contrast-safe). On dark, gold is high-contrast and used freely.

## Typography
Self-hosted variable woff2 in `public/fonts/` (no external requests). All three are
off the impeccable reflex-reject list; paired on a contrast axis.

- **Display** — `Bricolage Grotesque` (700–800): headings, project names, contact.
  Characterful grotesque; carries the voice.
- **Body** — `Hanken Grotesk` (400–600): prose. Calm, legible humanist sans.
- **Data/mono** — `Spline Sans Mono` (400–500): nav, kickers, tech tags, years,
  coordinates. Mono is real "instrument data," not decorative developer costume.

Scale: fluid `clamp()`, display letter-spacing `-0.03/-0.025em` (≥ -0.04 floor),
hero max 5rem (≤ 6rem ceiling), `text-wrap: balance` on headings.

## Signature element — the lensing canvas
`public/js/hero-lens.js`. A square lattice bent by a point-mass thin-lens map
(`r' = ½(r + √(r²+4Rₑ²))`), an Einstein ring lit where deflection peaks, over a
faint starfield. Load: Rₑ eases in over 1.5 s (the lens "turns on"). Idle: slow
field rotation, ring breathe. Pointer parallax on fine pointers. `prefers-reduced-
motion` → one static frame. Lattice segments brighten near the ring (magnification).

## Motion
`--ease: cubic-bezier(0.22,1,0.36,1)` (ease-out-quart). Reveals are progressive
(`[data-reveal]` hidden only when `.js` present, IntersectionObserver adds
`.is-visible`, staggered by `data-reveal-delay`, safety-net reveal after load). Nav
solidifies via IntersectionObserver on the hero. All motion has a reduced-motion
off-switch. `public/js/interactions.js`.

## Layout
- Container `max-width: 1120px`, fluid `--gutter: clamp(1.25rem,4vw,3rem)`.
- **Work** is an editorial index (`.work-row`): asymmetric 7fr/11fr rows, hairline
  dividers, flagship (Lensing) enlarged. Not a card grid. Collapses to 1 col ≤720px.
- **Skills** is an aligned "instrument legend" (`.legend-row`, 5fr/13fr).
- **Research** in a 760px measure: publications (year / title / venue) + tooling.
- Section rhythm without eyebrows: display title + one lede line; groups labelled by
  a lowercase mono line with a gold diamond and a rule.

## Components / conventions
- Buttons: `.btn` (mono, 2px radius); `.btn-primary` gold with dark ink + glow hover.
- Links: gold underline grow (nav), gold-deep with arrow slide (work/contact).
- Badges: `private` mono pill for private repos.
- z-index scale via `--z-nav` / `--z-overlay` (no magic numbers).

## Files
`style/main.scss` (system), `src/components/*` (markup), `src/data/projects.rs`
(content), `public/fonts/*`, `public/js/{hero-lens,interactions}.js`.
