# Product

## Register

brand

## Users
Hiring managers, collaborators, and fellow engineers/researchers who land on the
site to quickly judge depth and range. They read at a desk, in daylight, deciding
in under a minute whether Eduardo is the real thing. The site is the artifact
being judged, so craft is the message.

## Product Purpose
A personal portfolio for **Eduardo Gonik** — a data & AI engineer (and Physical-
Chemistry PhD researcher at INIFTA/INTECH, La Plata). It presents a coherent
end-to-end story: data acquisition → embedding pipelines → a prediction framework
(**Lensing**) → the applications that ship models, grounded by a real
scientific-computing/research thread. Success = a visitor comes away convinced of
both **depth** (production systems, real publications) and **range** (Rust, Python,
Zig, Julia, OCaml; ML, systems, frontend).

## Brand Personality
Precise · experimental · quietly confident. The voice of someone who measures
things exactly and builds his own instruments. Not loud, not salesy; the substance
carries it. Three physical-object words: **instrument-precise, luminous, measured.**

## Anti-references
- Generic SaaS landing pages: indigo/violet accent on white, Inter, restrained
  "one accent ≤10%" minimalism, identical icon+title+text card grids.
- The two second-order reflexes for an engineer portfolio: **dark-terminal**
  ("developer → monospace neon on black") and **editorial-serif magazine**
  (display-serif italic + drop caps + broadsheet grid).
- Tiny uppercase tracked eyebrows above every section; numbered section markers.
- Anything that reads as "which AI generated this?".

## Design Principles
1. **The concept is his own work.** The identity is gravitational lensing — light
   bending around mass — because it *is* Lensing (his flagship), *is*
   photoluminescence (his research), *is* the spectrogram (songViz). Metaphor over
   decoration.
2. **Show the range as one thread.** Projects read as a curated index that traces
   data → model → app, not a bag of cards.
3. **Instrument, not brochure.** Mono for real data (tech, years, coordinates);
   display type for voice. Precision is the aesthetic.
4. **Art-direct the arc.** Deep-field dark hero → precise light body → back into the
   field at contact. Consistency of voice over uniformity of treatment.
5. **Substance first.** Real repos, real metrics, real publications; the design
   frames them, never inflates them.

## Accessibility & Inclusion
Target WCAG AA. All text/background pairs verified ≥4.5:1 (body 7.5:1 on paper,
muted 8.2:1 on the dark field, gold button 9.7:1). The lensing canvas is decorative
(`aria-hidden`) and content never depends on it. Every animation (canvas, reveals,
scroll cue) has a `prefers-reduced-motion` path that renders a single static state.
Content is visible by default; JS only enhances.
