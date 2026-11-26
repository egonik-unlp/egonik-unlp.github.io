---
owner: Eduardo Gonik
last_updated: 2026-10-17
status: draft
---

# profile-page — decisiones

Rationale completo en `openspec/changes/archive/2026-10-17-replace-portfolio-with-profile-page/design.md`.

| Decisión | Por qué |
|---|---|
| Página de perfil sobria en vez del portfolio "Lensing field" | El sitio anterior se leía como hecho por IA (estilo pesado, Rust al frente, prosa de IA, 28 proyectos). El público (recruiters, líderes técnicos) busca sustancia verificable, no oficio de CSS. |
| Texto escrito por entrevista | Para evitar prosa de IA: el autor responde, el texto se arma solo con sus respuestas o el CV. |
| Zola, no Leptos ni HTML plano | Markdown para escribir prosa; i18n incluido; CI simple. Leptos dejaba el texto en literales de Rust; HTML plano duplicaba cada cambio. |
| Zola 0.22.1, no 0.23 | 0.23 pasó a Tera 2 y los templates usan sintaxis de Tera 1 (con 0.23 fallan los macros). |
| Prosa en el front matter; publicaciones en `data/publications.toml` | Una página sola no gana nada separando prosa; las publicaciones son compartidas entre idiomas y salen de Scholar. |
| Imágenes opcionales con `get_image_metadata(allow_missing)` | Un placeholder visible en producción es justo el rastro que se quería evitar. |
| Stack siempre visible, con etiquetas | La versión colapsable se vio mal (en parte por CSS cacheado); el usuario pidió probar sin colapsar. |
| `cachebust` en el link del CSS | Pages cachea el CSS 10 minutos; sin hash, HTML nuevo + CSS viejo rompe el layout tras un deploy. |
| Imagen de lvv: logotipo con las letras como vector | *l v v* en itálica matemática dentro de celdas de memoria, con flechas de vector sobre las v. |
| Imagen de convert-invert: búsqueda entre pares | Sale de lo que hace el proyecto, con color propio; no reusa el lenguaje de lvv ni el nombre completo. |
| Link a Scholar sin `hl` | Mismo link saliente en los dos idiomas; Scholar elige el idioma del visitante. |
| Sin Docker/Render; deck indexable | El sitio solo se publica en Pages; las slides se enlazan, así que se dejan indexar. |
