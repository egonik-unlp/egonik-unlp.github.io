# Tasks: replace-portfolio-with-profile-page

<!-- Tasks de implementación en orden. Ver design.md para el approach técnico. -->

## Implementation

- [x] 1. Trabajar sobre una rama que incluya los commits de la página de snappec-verify (la rama
      actual `snappec-verify-page`, o `main` una vez mergeada) para no perder `public/snappec-verify/`.
- [x] 2. Mover los estáticos que se conservan a `static/` con `git mv` y agregar `public/` a
      `.gitignore` (design.md → Archivos estáticos y URLs preservadas).
- [x] 3. Copiar `~/Documents/cv-edu-9-26.pdf` a `static/cv.pdf`.
- [x] 4. Crear `config.toml` con inglés por defecto, `[languages.es]` y las traducciones de etiquetas
      (design.md → Generador).
- [x] 5. Crear `data/publications.toml` con las 8 publicaciones de "Texto acordado" (año, título,
      revista, tipo, `first_author`, `status`).
- [x] 6. Crear `content/_index.md` con el texto acordado en inglés: párrafo de perfil en el cuerpo,
      el resto en `[extra]` (design.md → Dónde vive cada texto).
- [x] 7. Pedirle al usuario sus correcciones del español (sondear/perfilar, tracing, voseo, título
      docente, tesina, tema/canción, nombre del doctorado) y crear `content/_index.es.md` con el texto
      corregido.
- [x] 8. Crear `templates/index.html` y `templates/macros.html` con la disposición B: encabezado,
      perfil, Work, Projects (Infinite Playlist anidado), Research, footer, selector de idioma,
      `lang`, `hreflang` y meta description (ui-ux.md → Componentes y flujos).
- [x] 9. Implementar la macro de imagen opcional con `get_image_metadata(..., allow_missing=true)` y
      `resize_image`, con `width`/`height` y `alt` desde el front matter (design.md → Imágenes
      opcionales).
- [x] 10. Escribir `static/style.css` según design.md → Estilo, con foco visible y touch target del
      CV (ui-ux.md → Accesibilidad).
- [x] 11. Reemplazar `static/favicon.svg` por un monograma "EG" plano en el color de acento.
- [x] 12. Quitar `Disallow: /lensing-presentation/` de `static/robots.txt`.
- [x] 13. Capturar las imágenes que se puedan obtener (demo en vivo de Infinite Playlist, una slide
      del deck de Lensing), guardarlas en `static/img/` y cargar sus `alt` en ambos idiomas. Dejar
      anotado cuáles faltan (foto, lvv, traza de Jaeger).
- [x] 14. Cambiar la salida por defecto de `scripts/publish-snappec-verify.sh` a
      `static/snappec-verify` y actualizar su comentario.
- [x] 15. Reescribir `.github/workflows/gh-pages-deploy.yml` para Zola con versión fijada (build en
      PRs, deploy solo en `main`) (design.md → Deploy).
- [x] 16. Eliminar `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `src/`, `style/`,
      `public/js/`, `public/fonts/`, `public/assets/data/`, `Dockerfile`, `.dockerignore`,
      `render.yaml`, `PRODUCT.md` y `DESIGN.md`.
- [x] 17. Reescribir `README.md`: qué es el sitio, cómo editar el texto y las publicaciones, `zola
      serve`/`zola build`, cómo publicar snappec-verify.

- [x] 18. Agregar la sección Stack (pedido posterior al merge): `[[extra.stack]]` en ambos idiomas con
      la lista de design.md → Decisiones tomadas en apply, etiquetas en `config.toml`, el bloque
      `<details>` entre Projects y Research en el template y su CSS.

- [x] 19. Rehacer la sección Stack con la opción S2 (etiquetas, control Show/Hide, leyenda) y moverla entre el perfil y Work.
- [x] 20. Dejar la sección Stack siempre visible (sin `<details>`) y agregar `cachebust=true` al link del CSS.

## Validation

- [x] Stack: visible al cargar sin interacción, mismos ítems y niveles en
      `/` y `/es/`, sin desborde a 360px, Lighthouse de accesibilidad sin regresiones.

- [x] `zola check` sin errores (links internos y externos) y `zola build` limpio con la versión
      fijada.
- [x] El árbol de `public/lensing-presentation/` y `public/snappec-verify/` en la salida es idéntico
      byte a byte al de la rama base (`diff -r` contra un checkout de la base).
- [x] `install.sh` instala desde el build servido localmente (`SNAPPEC_VERIFY_BASE` apuntando a
      `zola serve`) en un directorio temporal.
- [x] Correr `scripts/publish-snappec-verify.sh` con `ALLOW_DIRTY=1` hacia un directorio temporal o
      revisar que su ruta por defecto sea `static/snappec-verify`.
- [x] `/` y `/es/` tienen las mismas secciones, entradas y links salientes; cada una enlaza a la
      otra; `lang` correcto en cada una.
- [x] Cada oración visible está en "Texto acordado" de `context.md` (o en las correcciones del
      usuario para el español); pasada de unslop en modo check sobre ambos idiomas sin hallazgos P0/P1.
- [x] La salida no contiene `<script>`, animaciones ni `@font-face`; la página se lee completa con
      JavaScript desactivado.
- [x] A 360px de ancho no hay scroll horizontal; el link del CV mide ≥ 44px de alto.
- [x] Accesibilidad: orden de títulos h1→h2→h3→h4, foco visible en todos los links, contraste ≥ 4.5:1
      en texto y botón, `alt` en las imágenes presentes, nombres accesibles distintos para cada
      "repo"/"slides"/"live demo" (chequeo manual + axe o Lighthouse).
- [x] Con una imagen faltante no aparece marco, placeholder ni imagen rota.
- [x] `/cv.pdf` descarga el CV; `/assets/data/projects.json` no existe en la salida;
      `robots.txt` no bloquea el deck.
- [x] El workflow pasa en el PR; después del merge, Pages sirve el sitio nuevo en
      https://egonik-unlp.github.io/ y `/es/`.

## Post-archive

- [ ] `/documenter distill <dated-change-id>` — **auto-ejecutado por `/opsx:archive`**.
      Solo correr a mano si el auto-trigger falló (re-run con el id completo con fecha,
      ej. `2026-10-17-replace-portfolio-with-profile-page`). Sincroniza el use-case
      (knowledge/modules/<m>/use-cases/profile-page/) con los deltas del change (spec narrativa,
      decisiones, lore). Humano revisa y acepta edits propuestos.
