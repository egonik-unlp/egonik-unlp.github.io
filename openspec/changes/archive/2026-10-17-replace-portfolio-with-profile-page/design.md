# Design: replace-portfolio-with-profile-page

## Approach

> Sin grafo: Graphify no estaba disponible (ver `context.md` → Nodos afectados). Las áreas se
> nombran como en esa tabla en lugar de `graph://` refs.

### Generador: Zola, una página por idioma

El sitio pasa a ser un build estático de Zola. Es una sola página, así que el contenido vive en dos
archivos de sección:

- `content/_index.md` → `/` (inglés, idioma por defecto)
- `content/_index.es.md` → `/es/` (español)

`config.toml` declara `default_language = "en"`, `[languages.es]`, `compile_sass = false`,
`build_search_index = false` y las traducciones de etiquetas fijas (`[translations]` / `[languages.es.translations]`:
"Work", "Projects", "Research", "first author", "submitted", "repo", "live demo", "slides", etc.).

### Dónde vive cada texto

- **Front matter `[extra]` de cada `_index*.md`:** todo lo que se traduce y tiene forma de lista:
  encabezado (headline, links), `[[extra.work]]` (org, rol, período, párrafos, tecnologías),
  `[[extra.projects]]` (nombre, descripción, links, imagen, `alt`, hijos para Infinite Playlist),
  el párrafo de investigación, la codirección y `[[extra.lab_software]]`.
- **Cuerpo Markdown de cada `_index*.md`:** el párrafo de perfil. Es el texto que más va a editar el
  usuario y Markdown es el formato natural para eso.
- **`data/publications.toml` (compartido):** las 8 publicaciones (año, título, revista, tipo,
  `first_author`, `status`). Los títulos no se traducen y Google Scholar es la fuente de verdad: un
  solo archivo para actualizar. El template lo carga con `load_data` y traduce solo las etiquetas.

El texto se copia tal cual de "Texto acordado" en `context.md`; el español se carga con las
correcciones del usuario antes de dar el change por cumplido.

### Templates

- `templates/index.html`: la página completa con la disposición B (encabezado con foto, perfil,
  Work, Projects, Research, footer). `lang` en `<html>` desde el idioma de la página; `<link
  rel="alternate" hreflang>` para `en` y `es`; meta description por idioma.
- `templates/macros.html`: macros para una entrada de Work, una de proyecto (con hijo opcional) y una
  imagen opcional.

### Imágenes opcionales

Las imágenes fuente van en `static/img/` (`photo`, `lensing`, `infinite-playlist`, `lvv`,
`convert-invert`). La macro de imagen llama a `get_image_metadata(path=..., allow_missing=true)`: si
no existe el archivo, no emite nada (ni marco ni placeholder), lo que cumple el requirement "Images
are optional". Si existe, genera la miniatura con `resize_image` (ancho 2× del mostrado, `webp`) y
escribe `width`/`height` para evitar saltos de layout, con el `alt` del front matter.

En apply se capturan las imágenes que se puedan obtener sin el usuario: la demo en vivo de Infinite
Playlist y una slide del deck de Lensing. Foto, lvv y la traza de Jaeger quedan para después.

### Estilo

Un `static/style.css` escrito a mano, chico: fuentes del sistema (`system-ui` y pila equivalente),
fondo blanco, tinta casi negra, un acento `#1f5f4a` en una custom property, separadores finos, layout
con flex/grid que colapsa a una columna. Nada de Sass, fuentes web, animaciones ni JS. Touch target de
44px en el link del CV.

### Archivos estáticos y URLs preservadas

Zola copia `static/` tal cual a la raíz del sitio, así que mover con `git mv`:

- `public/lensing-presentation/` → `static/lensing-presentation/` (rutas relativas, sin cambios)
- `public/snappec-verify/` → `static/snappec-verify/` (página `noindex`, `install.sh`, tarballs)
- `public/robots.txt` → `static/robots.txt`, sin el `Disallow: /lensing-presentation/`
- `public/favicon.svg` → `static/favicon.svg`, reemplazado por un monograma "EG" plano en el color
  de acento (el actual es el anillo dorado del concepto "Lensing field"). *Supuesto: detalle menor
  fuera del brief.*
- `~/Documents/cv-edu-9-26.pdf` → `static/cv.pdf`

Zola escribe su salida en `public/` por defecto. Como `public/` deja de ser fuente, pasa a
`.gitignore`. `scripts/publish-snappec-verify.sh` cambia su salida por defecto a
`$site/static/snappec-verify` (y el comentario de cabecera).

### Deploy

El workflow de Pages se reduce a: checkout → instalar Zola en una versión fijada → `zola build` →
subir `public/` → deploy (solo en `main`; en PRs se mantiene el build como chequeo). Se van el
toolchain de Rust, `cargo-leptos`, Dart Sass, `cargo fmt --check` y el paso que levantaba el servidor
para curlear `/` y `/es`.

### Limpieza

Se eliminan el código y la configuración de Leptos, `style/`, `public/js`, `public/fonts`,
`public/assets/data`, `Dockerfile`, `.dockerignore`, `render.yaml`, `PRODUCT.md` y `DESIGN.md`. El
`README.md` se reescribe: qué es el sitio, cómo editar el texto (los dos `_index*.md` y
`data/publications.toml`), cómo correrlo (`zola serve`), cómo publicar snappec-verify.

## Archivos a modificar

| Archivo | Cambio |
|---|---|
| `config.toml` | nuevo: Zola, idiomas en/es, traducciones de etiquetas |
| `content/_index.md`, `content/_index.es.md` | nuevos: front matter `[extra]` + párrafo de perfil |
| `data/publications.toml` | nuevo: las 8 publicaciones |
| `templates/index.html`, `templates/macros.html` | nuevos: disposición B |
| `static/style.css` | nuevo: CSS mínimo |
| `static/img/*` | nuevos: imágenes que se puedan capturar |
| `static/cv.pdf` | nuevo: copia de `cv-edu-9-26.pdf` |
| `static/lensing-presentation/**` | movido desde `public/`, sin cambios |
| `static/snappec-verify/**` | movido desde `public/`, sin cambios |
| `static/robots.txt` | movido; sin `Disallow` del deck |
| `static/favicon.svg` | movido y reemplazado por monograma plano |
| `scripts/publish-snappec-verify.sh` | salida por defecto → `static/snappec-verify` |
| `.github/workflows/gh-pages-deploy.yml` | reescrito para Zola |
| `.gitignore` | agrega `public/` (salida de Zola) |
| `README.md` | reescrito |
| `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `src/**` | eliminados |
| `style/main.scss` | eliminado |
| `public/js/*`, `public/fonts/*`, `public/assets/data/*` | eliminados |
| `Dockerfile`, `.dockerignore`, `render.yaml` | eliminados |
| `PRODUCT.md`, `DESIGN.md` | eliminados |

## Alternativas consideradas

- **Mantener Leptos y quitar la hidratación.** Descartada: el texto seguiría en literales de Rust
  (`t("…", "…")` con saltos escapados), incómodo para que el usuario escriba prosa, y el CI seguiría
  levantando un servidor para renderizar. El usuario eligió Zola.
- **HTML plano, un archivo por idioma.** Descartada: cada cambio de estructura se hace dos veces y no
  hay procesamiento de imágenes ni chequeo de imágenes faltantes.
- **Todo el contenido en archivos de datos (`load_data`) en vez de front matter.** Descartada para la
  prosa: separa el texto de su página sin ganar nada en una sola página. Se usa solo para
  publicaciones, que son compartidas y vienen de Scholar.
- **Leer el contenido del databank de `project-cv`.** Diferida (non-goal). El databank ya tiene
  registros bilingües y vistas, pero conectarlo es un cambio aparte; la estructura de front matter
  elegida no lo impide.
- **Placeholders visibles para imágenes faltantes.** Descartada: un recuadro "[Your photo]" en
  producción es exactamente el tipo de rastro que el usuario quiere evitar.

## Decisiones tomadas en apply

- **Zola 0.22.1, no 0.23.x.** La 0.23 migró a Tera 2 y la documentación de Zola solo cubre Tera v1
  ("Zola 0.22 and before use Tera v1"); con 0.23.6 la sintaxis de macros de Tera 1 falla ("Unknown
  tag"). Se fija 0.22.1 (enero 2026), que tiene `get_image_metadata(allow_missing)` y `resize_image`.
  Migrar a 0.23 es un cambio aparte.
- **URLs marcadas `| safe` en los templates.** El autoescape de Tera escribe `/` como `&#x2F;`
  dentro de atributos; es HTML válido pero ilegible. Las URLs vienen del front matter propio.
- **Link a Scholar sin `hl`.** Con `hl=en` / `hl=es` los dos idiomas tenían un link saliente
  distinto; sin el parámetro el link es idéntico (como pide la spec) y Scholar elige el idioma del
  visitante.
- **`static/processed_images/` en `.gitignore`.** Zola 0.22 guarda ahí el caché de `resize_image`;
  el CI lo regenera en cada build.
- **`overflow-wrap: break-word` en `body`.** Los títulos de publicaciones tienen palabras largas
  (p. ej. "Perylene-3,4,9,10-Tetracarboxylic") que no deben forzar scroll horizontal a 360px.
- **Sección Stack agregada después del merge (pedido del usuario, scope nuevo sobre la proposal
  aprobada).** Colapsada con `<details>` nativo (sin JS). Primera versión: entre Projects y Research, con una línea gris "Also used"; el usuario la encontró fea. Versión final (opción S2 del canvas): entre el perfil y Work, encabezado entre filetes con un control "Show"/"Hide" a la derecha, y por grupo etiquetas rellenas (uso regular) seguidas de etiquetas con borde (también usado) más una leyenda. Cada tipo es su propia lista con `aria-label` para lectores de pantalla. Fuente: la
  unión de la lista del sitio anterior (`src/components/skills.rs`) y los `skills` del databank de
  `project-cv`. Niveles: los ítems del databank toman su `level` (`primary` → regular; `working` y
  `familiar` → "also used", como el negrita/gris del CV); el usuario ubicó el resto. Deep learning
  pasa a ML & AI. Vive en `[[extra.stack]]` del front matter de cada idioma. Lista acordada:

  | Grupo | Uso regular | También usado |
  |---|---|---|
  | Languages | Rust, JavaScript / TypeScript, Python, Julia | Zig, Go, OCaml |
  | ML & AI | LangChain, LangGraph, Embeddings, RAG, PyTorch, Deep learning | Keras, burn, Qdrant, Chroma, LlamaIndex, Flux.jl, scikit-learn / XGBoost, ONNX, Langfuse |
  | Data | PostgreSQL, Mathematical modelling | Redis, Julia data ecosystem, ETL pipelines, pandas / polars, Job queues, Web scraping |
  | Systems & Web | Axum, Express, tokio, Diesel | Actix Web, Leptos, Flask, OpenTelemetry / Jaeger, Multi-worker systems |
  | Infra & Platforms | Linux, Docker & Compose | Windows, macOS, Cloudflare Workers, Portainer, AWS & GCP |

- **Stack sin colapsar.** La versión colapsable (S2) se vio rara en producción: el navegador mezcló el HTML nuevo con el `style.css` anterior, cacheado 10 minutos por el CDN de Pages. El usuario pidió probar sin colapsar: la sección queda siempre visible, con `h2` como las demás, las etiquetas y la leyenda entre filetes. Además, el link al CSS lleva `cachebust=true` (hash en la URL), así cada deploy fuerza un CSS nuevo.

- **Imagen de lvv: logotipo C1.** El usuario descartó una captura de código y una serie de ilustraciones; eligió un logotipo donde las letras son la representación del vector: *l v v* en itálica matemática dentro de tres celdas de memoria contiguas (offsets 0x0–0x2), con flechas de vector sobre las v. Se renderiza a `static/img/lvv.png` (tarjeta oscura 16:10 como las otras miniaturas).

- **Imagen de convert-invert: búsqueda entre pares, paso 4 de 5.** Se descartó reusar el lenguaje de lvv (la matemática no describe a convert-invert) y el nombre completo. El motivo sale de lo que hace: una red de pares con la ruta de búsqueda resaltada hasta el par que tiene el archivo, más pares inalcanzables punteados. Color propio (rosa `#f28dc4`) en vez del verde de lvv. Se renderiza a `static/img/convert-invert.png` como tarjeta oscura 16:10.

- **Primer `zola build` borró `public/`.** Zola limpia su directorio de salida; los archivos
  trackeados que quedaban ahí (JSON, fuentes, JS) eran justamente los que la task 16 elimina.

## Riesgos

- **Versión de Zola:** `get_image_metadata(..., allow_missing=true)` y `resize_image` dependen de la
  versión. Fijar la misma versión en el workflow y en local, y verificar que la elegida los soporte.
- **`public/` cambia de rol:** pasa de fuente a salida. Si quedara algún archivo fuente sin mover,
  Zola lo pisa en cada build; mover todo antes del primer `zola build`.
- **URLs byte a byte:** `install.sh` y el deck dependen de rutas fijas. Comparar el árbol de
  `static/lensing-presentation` y `static/snappec-verify` contra el de `main` después del build.
- **`/es` → `/es/`:** los links internos y el selector de idioma apuntan a `/es/`.
- **Contenido pendiente del usuario:** sin las correcciones del español el change no cumple sus
  criterios; la foto y las capturas faltantes no bloquean (las imágenes son opcionales).
- **CV:** se publica tal cual; si el usuario regenera el PDF, alcanza con reemplazar `static/cv.pdf`.
- **Resultados viejos en buscadores:** URLs como `/assets/data/projects.json` van a dar 404; es lo
  buscado.
