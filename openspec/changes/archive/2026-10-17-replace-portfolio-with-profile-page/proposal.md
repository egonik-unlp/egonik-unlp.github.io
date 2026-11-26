# Proposal: Reemplazar el portfolio por una página de perfil

**Change ID:** replace-portfolio-with-profile-page
**Date:** 2026-10-17
**Author:** Eduardo Gonik
**Estado:** Aprobado — 2026-10-17
<!-- Marca del gate de aprobación. La escribe SOLO /opsx:propose, y solo tras un Approve explícito
     del usuario en el gate: "Aprobado — <fecha>". Toda (re)generación de este archivo la resetea a
     "Pendiente de aprobación" — se aprueba un texto, no un archivo. -->

## Why

A egonik-unlp.github.io llegan sobre todo recruiters que hacen drill-down desde el CV (el PDF enlaza
el sitio) y líderes técnicos capaces de juzgar el mérito técnico. El sitio actual se lee como hecho
por IA: estilo pesado (canvas animado, tres fuentes propias, fondo crema con acento naranja), Rust al
frente ("Built in Rust", el stack como identidad), descripciones de proyectos con prosa de IA, una
estructura de landing genérica (hero con botones, grilla de skills, "Let's talk") y 28 proyectos
organizados en dos taxonomías inventadas, sin nada destacado. Para un perfil que no es de frontend,
ese envoltorio contradice lo que quiere mostrar.

El objetivo es una página de perfil sobria que muestre su perfil poco estándar (ingeniería,
investigación y docencia en paralelo) y sus capacidades técnicas con sus propias palabras, sin
jactancia. Ahora, porque el CV se suma al sitio y los recruiters llegan desde el CV.

## Scope

Capability nueva: **`profile-page`** (la página de perfil bilingüe y los recursos estáticos que la
acompañan).

> Sin grafo: el MCP de Graphify no conectó y el repo no tiene `graphify-out/` ni `knowledge/`. Las
> áreas y archivos salen de `context.md`, que los obtuvo leyendo el repo.

**Comunidades afectadas:** (ver context.md → Nodos afectados) app Leptos (shell, router,
componentes, datos), estilos y JS de cliente, JSON legados, deploy, docs. Se conservan sin cambios
los subsitios `lensing-presentation` y `snappec-verify`.

**Archivos principales:**

- Se eliminan: `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `src/**`, `style/main.scss`,
  `public/js/*`, `public/fonts/*`, `public/assets/data/*.json`, `Dockerfile`, `.dockerignore`,
  `render.yaml`, `PRODUCT.md`, `DESIGN.md`.
- Se crean: configuración de Zola con inglés (por defecto) y español, un template para la página,
  el contenido acordado en ambos idiomas, un CSS mínimo, `static/` con el deck, snappec-verify,
  `cv.pdf` e imágenes.
- Se modifican: `.github/workflows/gh-pages-deploy.yml` (build con Zola),
  `scripts/publish-snappec-verify.sh` (ruta de salida), `public/robots.txt` (sin `Disallow` del
  deck), `README.md` (reescrito), `public/favicon.svg` (revisar).

**Contenido** (fuente: "Texto acordado" en `context.md`, producto de la entrevista):

- Encabezado: nombre, "Data engineer · La Plata, Argentina", correo, GitHub, LinkedIn, Google
  Scholar, botón "CV (PDF)" a `/cv.pdf` (el archivo `cv-edu-9-26.pdf`), selector de idioma.
- Perfil: el párrafo de intro acordado.
- Work: Snappler (sin nombrar al cliente de AFALP), Goodvibes Movement, docencia en la UNLP.
- Projects: Lensing con Infinite Playlist debajo, lvv, convert-invert, "More on GitHub".
- Research: doctorado (estado: tesis en escritura), codirección de tesina, 8 publicaciones con link a
  Google Scholar como fuente de verdad, 5 entradas de software del laboratorio.

**Disposición y aspecto** (opción B elegida en la exploración): orden de CV; foto junto al nombre;
miniaturas chicas por proyecto (Infinite Playlist más chica, bajo Lensing); publicaciones y software
del laboratorio lado a lado; títulos sans; fuentes del sistema; fondo blanco; un acento verde
(#1f5f4a, asumido); sin animaciones; la página no requiere JavaScript.

**Imágenes:** se capturan las que se puedan obtener (demos en vivo, una slide del deck); la foto y el
resto se agregan después. Mientras falten, la disposición B se mantiene sin mostrar huecos vacíos.

**Otros ajustes:** `robots.txt` deja de bloquear `/lensing-presentation/`.

## Impact

- **URLs que se mantienen:** `/`, `/es` (servida como `/es/`), `/lensing-presentation/**` y
  `/snappec-verify/**` (incluidos `install.sh`, `latest/` y `versions/0.1.0/`, que `install.sh`
  descarga por URL fija). Tienen que responder igual que hoy.
- **URLs nuevas:** `/cv.pdf`.
- **URLs que desaparecen:** `/assets/data/projects.json`, `projects.es.json`, `works.json`,
  `works.es.json`, y los assets de Leptos (`/pkg/*`, `/js/*`, `/fonts/*`). El README de `project-cv`
  enlaza `projects.json` y queda roto; su copia transcripta no se ve afectada.
- **Deploy:** el workflow deja de compilar Rust, de levantar el servidor y de curlear rutas; pasa a
  un `zola build`. Desaparece el despliegue en contenedor (Docker/Render).
- **Publicación de snappec-verify:** el script tiene que escribir en el nuevo directorio estático;
  si no, la próxima versión queda fuera del sitio.
- **Indexación:** el deck pasa a ser indexable.
- **Coherencia con el CV:** el sitio y el PDF comparten hechos; el PDF tiene typos y una línea
  ("A full rewrite of the ingredient recognition app") que no coincide con la entrevista. Se publica
  tal cual salvo que el usuario lo regenere.

## Non-goals

- Leer el contenido desde el databank de `project-cv` (queda como posibilidad futura).
- Destacar cv-data como proyecto.
- Recursos de docencia en el sitio.
- Nombrar al cliente de AFALP o enlazar su código.
- Reescribir la historia del repo.
- Una página índice de proyectos o el catálogo de 28 proyectos.
- Enlazar la página de snappec-verify desde el perfil.
- Corregir o regenerar el CV.

## Criterios de aceptación

- Cada oración visible proviene de las respuestas de la entrevista o del CV; no hay texto de relleno
  ni afirmaciones nuevas.
- `/` muestra la página en inglés y `/es/` en español, con el mismo contenido y estructura, y cada
  una enlaza a la otra.
- El español incorpora las correcciones del usuario.
- El botón "CV (PDF)" del encabezado descarga `cv.pdf`.
- La página se lee completa sin JavaScript y sin animaciones, y funciona a ancho de teléfono sin
  scroll horizontal.
- Los links de demos y repos de los proyectos funcionan; Google Scholar figura como fuente de la
  lista de publicaciones.
- `/lensing-presentation/` y `/snappec-verify/` (con `install.sh` y los tarballs) responden igual que
  antes del change.
- El build de Zola corre en el workflow y despliega en GitHub Pages.
- Ya no existen en el repo el código Rust/Leptos, `style/`, los JS de cliente, las fuentes propias,
  los JSON legados, `Dockerfile`, `.dockerignore`, `render.yaml`, `PRODUCT.md` ni `DESIGN.md`.
- `robots.txt` ya no bloquea `/lensing-presentation/`.

## Conflictos con decisiones existentes

No hay `knowledge/decisions.md` contra qué chequear. Sí hay conflicto con la intención documentada en
el repo (ver context.md → Riesgos identificados):

- **`PRODUCT.md` y `DESIGN.md`** fijan la dirección opuesta: "the site is the artifact being judged,
  so craft is the message", el concepto visual "Lensing field", canvas y animaciones. Este change la
  reemplaza explícitamente y elimina ambos documentos.
- **`README.md`** documenta el stack Leptos/Axum y el despliegue en contenedor; se reescribe.
- **`robots.txt`** bloqueaba el deck a propósito; el usuario decidió permitir la indexación.
