---
owner: Eduardo Gonik
last_updated: 2026-10-17
status: draft
---

# profile-page — notas

## Gotchas
- **`public/` es salida de Zola**, no fuente: `zola build` la borra entera. Los estáticos van en `static/`.
- **`static/processed_images/`** es el caché de `resize_image`; está en `.gitignore` y el CI lo regenera.
- **CSS cacheado en Pages** (`max-age=600`): tras el deploy del Stack colapsable, el navegador mezcló
  HTML nuevo con CSS viejo. Por eso el link al CSS lleva `cachebust`.
- **Las ramas nuevas** creadas desde `origin/main` quedan con upstream `origin/main`; empujar siempre con
  `git push -u origin HEAD:refs/heads/<rama>`.
- **El hook de graphify** (post-commit / post-checkout) regenera `graphify-out/` en cada commit.

## Contenido
- El español quedó como el borrador de traducción, sin correcciones (decisión del autor).
- El CV (`static/cv.pdf`) se publica tal cual: tiene typos ("responsabilities", "consumed an presented",
  "Express" duplicado) y la línea "A full rewrite of the ingredient recognition app", que no coincide
  con la entrevista (la reescritura fue de la herramienta de reportes).
- La lista de publicaciones se verificó contra el databank del CV; Scholar bloquea el acceso automático,
  así que el chequeo contra Scholar es manual.
- convert-invert: el README del repo muestra workers que se loguean a una red P2P; un lector puede
  leerlo como herramienta de descarga de música. Aceptado; el texto habla solo de la ingeniería.

## TODOs
- `<TODO>` Foto: agregar `static/img/photo.jpg`; el encabezado la muestra sola cuando existe.
- `<TODO>` Posible fuente futura del contenido: el databank bilingüe de `~/Documents/git/project-cv`
  (diseñado para alimentar este sitio). Quedó fuera de este change.
