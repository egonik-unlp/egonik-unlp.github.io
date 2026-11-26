# Context: replace-portfolio-with-profile-page

<!-- Consolidar contexto ANTES de escribir la proposal.
     Este artefacto reduce tokens de exploración downstream.

     El archivo tiene DOS MITADES con reglas distintas. El dueño de las dos es /opsx:explore: lo crea
     en su cierre convergente y es también quien lo itera (no hay comando de refresh aparte).
     /opsx:propose es colaborador acotado SOLO de la mitad conversacional: puede crear el archivo si no
     existe (camino directo) y appendear a la entrevista las Q&A que él mismo condujo — nunca toca la
     mitad derivada (deja los marcadores _Pendiente_).
     1. "Entrevista de intención" — conversacional. La escribe quien condujo la entrevista y crece por
        APPEND: las Q&A nuevas van al final; las ya registradas no se reescriben, reordenan ni borran.
        El intent brief se actualiza con respuestas reales, nunca se re-sintetiza de la conversación.
     2. "Nodos afectados" / "Archivos relevantes" / "Contexto knowledge" / "Riesgos identificados" —
        snapshot derivado de Graphify + knowledge/. Es regenerable: se reescribe completo al iterar.
        Si todavía no se consolidó, cada una de esas 4 secciones lleva el marcador
        `_Pendiente — correr /opsx:explore <change-id> para consolidar el snapshot técnico._`
        en lugar de quedar vacía (una sección vacía se lee como "no hay nodos / no hay riesgos"). -->

## Entrevista de intención

Conducida en /opsx:explore. La exploración tuvo una primera ronda (rediseño visual, 14 paneles) que el
usuario descartó por completo ("start over completely" → "Everything"); solo se registran las Q&A
posteriores al reinicio, que son las que sostienen este change.

### Q&A

| Pregunta | Respuesta |
|---|---|
| ¿Qué tiene que pasar después de que alguien visita el sitio? | Un recruiter que hace drill-down desde una lista de candidatos (ya tiene acceso a sus links) no debería encontrarse un sitio con prosa de IA. Mostrar el perfil y las capacidades técnicas sin ser jactancioso. El perfil no es estándar y eso puede ser una ventaja. Un link para descargar el CV; el CV se agrega en este refactor. |
| ¿Quién es el visitante que más importa? | Recruiter, y también un líder técnico: alguien desconfiado de proyectos excesivamente hechos con IA pero capaz de entender el mérito técnico. Las descripciones de proyectos pueden delatar la intervención de IA. Frontend/CSS no es su área (sí: herramientas, backend, entendimiento y modelado de datos); un sitio excesivamente estilizado no tiene sentido y que una IA lo estilice no impresiona a nadie. |
| Si esa persona recordara una sola frase sobre él, ¿cuál sería? | "Eduardo is interested in tool development, solution architecture and has ideas around data. He likes understanding how things work and can solve problems. He has a career in research and so he knows how to attack problems. His teaching experience gives him an ability to convey information." |
| ¿Qué es lo que menos le gusta del sitio actual? | Rust está demasiado al frente. Que una IA estilice el sitio no impresiona a nadie. Las explicaciones de proyectos son muy de IA. La estructura se siente como una landing genérica de IA. No busca algo hiperoriginal: no quiere huellas de IA por todo el proyecto. |
| ¿Qué debería agregar el sitio más allá del CV? | Links en vivo y demos; explorar las otras opciones. |
| ¿Cómo se escribe el texto? | Por entrevista: él responde con sus palabras, Claude recorta y ordena, él corrige. Pidió además que Claude retrabaje sus respuestas en un párrafo propio cuando la concatenación quedó mecánica. |
| ¿Se pueden nombrar Snappler y Goodvibes? | Sí. El cliente de AFALP no se puede nombrar; solo en términos abstractos. |
| ¿Cuál es la fuente de verdad de las publicaciones? | Google Scholar. El paper de ACS Catalysis es de 2026. |
| ¿El sitio debería ser una vista del databank de project-cv (el generador de su CV)? | Tal vez. |
| ¿Se destaca cv-data como proyecto? | No. |
| ¿Se mantiene la versión en español? | Sí. |
| Intro: ¿qué hacés en el trabajo? | Mayormente trabajo de desarrollo, relacionado con datos: analítica, modelado matemático y testing. También diseña y construye herramientas de probing y profiling. |
| Intro: ¿qué problema te gusta que te den? | AFALP es un ejemplo excelente (verify, probes, reconciliación). |
| Intro: ¿qué cambia tu formación en investigación? | Enfoques basados en evidencia; Lensing refleja un enfoque de modelado científico. |
| Intro: ¿qué querés que sepa un recruiter de tu docencia? | Experiencia transmitiendo información de forma que el público la entienda. Docencia mayormente en primer año: contenidos básicos, fuerte en recursos didácticos. Conocer la materia con más profundidad y a menor nivel de abstracción ayuda a explicar. |
| Feedback sobre los primeros borradores de intro | Se sentían como una concatenación de respuestas; pidió más trabajo, luego un único párrafo retrabajado con su información (sin variantes). |
| AFALP: ¿qué se puede decir? | Solo en abstracto. El trabajo empieza por primeros principios y lógica (verify) y construye hacia arriba (API → probe, interfaz → frontend probe). |
| ¿Profiling de qué? | De ambos, rendimiento y datos; se usan en probe y verify. |
| Lensing: ¿qué tiene de modelado científico? | Se propone un modelo, se entrena y se evalúa, se encuentran sus limitaciones y se usan para iterar. |
| Docencia: ¿qué cursos? | Dos cursos de Química General. |
| Goodvibes: ¿qué es? | Un proyecto que parte de conocimiento científico sobre ingredientes de productos; de ahí un sitio que da veredictos sobre productos de consumo y una app para fotografiar un producto y obtener información sobre ingredientes y sus riesgos. |
| Goodvibes: integridad y auditoría de datos | Datos de ingredientes y productos: buscó inconsistencias, sugirió e implementó correcciones. |
| Goodvibes: aproximación de ingredientes | Determinar los ingredientes reales de la etiqueta: primero por menor distancia de Levenshtein contra la base, después con bases vectoriales. |
| Goodvibes: la reescritura | La herramienta que parseaba bases de ingredientes y generaba los reportes XML que consumía el sitio, incluidos los puntajes de productos. Lo anterior era un script de R mal documentado (en el texto público quedó "an older R script"). |
| Proyectos: ¿qué ocupa el lugar de probing/profiling? | convert-invert. |
| ¿Recursos de docencia en el sitio? | No. |
| ¿Infinite Playlist como entrada propia o bajo Lensing? | Bajo Lensing. |
| Investigación en tus palabras | Diseño, desarrollo y caracterización de materiales metal-orgánicos nanométricos para terapia fotodinámica; la estructura y su correlación con procesos y dinámicas fotofísicas y fotoquímicas se estudiaron principalmente por espectroscopía. |
| Estado de la tesis | La parte experimental y la mayor parte del análisis están completas; está en escritura. |
| ¿Qué disposición? (A: orden CV con look de CV · B: orden CV con foto y miniaturas · C: tres tracks con timeline) | B. |
| Tipografía de títulos | Sans. |
| Imágenes (foto + 4 capturas) | Primero "later"; al cerrar: "get the images you can get" (Claude consigue las que pueda; el resto después). |
| ¿Cómo se construye? (Leptos estático / Zola / HTML plano) | Zola. |
| JSON legados (/assets/data/projects.json, works.json) | Eliminarlos. |
| ¿El CV es cv-edu-9-26.pdf servido en /cv.pdf? | Sí. |
| Cierre | "let's go"; nombre del change aceptado. |
| (propose) `Dockerfile`, `.dockerignore` y `render.yaml` solo sirven al servidor Leptos; con Zola no hacen nada. ¿Qué pasa con ellos? | Eliminarlos. |
| (propose) `robots.txt` bloquea `/lensing-presentation/`, pero Lensing va a enlazar las slides. ¿Qué hace `robots.txt`? | Permitir la indexación (quitar el `Disallow`). |

### Intent brief

**Intención:** The site is mostly reached by recruiters clicking through from his CV, and by technical leads who can judge technical merit. Right now it reads as AI-made: heavy styling, Rust up front, AI-written project descriptions, a generic landing-page structure, and 28 projects with no clear highlight. The goal is a plain profile page that shows his unusual profile (engineering, research and teaching) and his technical abilities in his own words, without boasting. Why now: the CV is being added to the site, and recruiters arrive from the CV.

**Alcance:** Rebuild the site in Zola using layout B:
- CV order: Profile, Work, Projects, Research.
- A photo and small project thumbnails, sans headings, green accent (#1f5f4a, assumed).
- The agreed copy (below) in English and Spanish (the Spanish still needs his corrections).
- The CV (`cv-edu-9-26.pdf`) at `/cv.pdf`, linked in the header.
- Images: the ones that can be captured now (live demos, a slide from the deck); the photo and the rest come later.
- Remove the old JSON files (`projects.json`, `works.json`) and the design docs (`PRODUCT.md`, `DESIGN.md`).
- Delete `Dockerfile`, `.dockerignore` and `render.yaml`; the site is deployed only to GitHub Pages. *(added in propose)*
- Remove the `Disallow: /lensing-presentation/` line from `robots.txt` so the deck can be indexed. *(added in propose)*
- Keep `/snappec-verify` and `/lensing-presentation` working at their current paths.

**Comportamiento observable:** `/` and `/es` each show one page:
- A header with his name, "Data engineer · La Plata, Argentina", contact links and a CV (PDF) button.
- The profile paragraph.
- Work: Snappler, Goodvibes, UNLP teaching.
- Projects: Lensing with Infinite Playlist under it, lvv, convert-invert, and "More on GitHub".
- Research: the PhD paragraph, the thesis co-direction, 8 publications with a Scholar link, and 5 lab software entries.

No animation, no JavaScript needed, and the old 28-project index is gone.

**Non-goals:**
- Reading content from the `project-cv` databank.
- Featuring cv-data.
- Teaching resources.
- Naming the AFALP client.
- Rewriting the repo's git history.
- A separate projects index page.

**Criterios de aceptación:**
- Every sentence traces back to his interview answers or the CV.
- English and Spanish are complete, and the Spanish has his corrections.
- The CV downloads from the header.
- The page works at phone width without JavaScript.
- The Zola build deploys to GitHub Pages.
- `/snappec-verify` (install script and tarballs included) and `/lensing-presentation` are served unchanged.
- `PRODUCT.md`, `DESIGN.md` and the old JSON files are removed.

**Incertidumbres abiertas:**
- The accent color is assumed, not confirmed.
- The Spanish wording choices need his review: sondear/perfilar, tracing, voseo, the teaching title, tesina.
- Which images can actually be captured.
- How convert-invert's peer-to-peer side will read to visitors.
- Whether the `project-cv` databank becomes the content source later.
- The CV's "full rewrite of the recognition app" line conflicts with his interview answer.
- He needs to check the publication list against Scholar by hand, since Scholar blocks automated access.
- Whether the site links to its own source repo, and whether the untracked `openspec/` and `.claude/` files get committed.

### Texto acordado

Resultado de la entrevista, copiado tal cual se acordó. Es la fuente del contenido para este change.
El español es un borrador de traducción pendiente de sus correcciones (ver Incertidumbres abiertas).

#### English


**Eduardo Gonik**
Data engineer · La Plata, Argentina
email · GitHub · LinkedIn · Google Scholar · CV (PDF)

I'm a data engineer. Most of my work is development around data (analytics, mathematical modelling and testing), and a good part of it is building tools to probe and profile systems and the data they handle. When I verify a system, I start from the logic of what has to hold and build upward, from the API to the interface. My background is in scientific research, and I still work that way: I propose a model, test it, find where it falls short, and use that for the next iteration. I also teach two general chemistry courses at UNLP, and knowing the subject in more depth, at a lower level of abstraction than the course, helps me explain it.

---

##### Work

**Snappler** · Data engineer · 2025–2026

In Snappler's AI Lab I built asynchronous and real-time data analysis pipelines, agentic systems, tracing for AI applications, and RAG search.

For a client's refactor, I built the verification in layers: first checking the logic of the change from first principles, then a probe that runs both versions of the API side by side on the same data, then a probe for the interface. Both probes also profile performance and data.

**Goodvibes Movement** · Software and data engineer · 2021–2026

Goodvibes builds on scientific knowledge about product ingredients. It runs a site that gives verdicts on consumer products, and an app where you photograph a product and get information about its ingredients and their risks. I was the only internal developer for five years.

I looked after the ingredient and product data, searching for inconsistencies and proposing and implementing fixes. I built the API that reads ingredients from label photos. OCR extracts the text, and each ingredient is matched to our database, at first by the smallest Levenshtein distance and later with a vector database. I also rewrote the reporting pipeline, replacing an older R script with a tool that parses the ingredient databases and generates the XML reports the site consumes, product scores included.

Rust · Axum · Leptos · Julia · Python · Flask · Docker · GCP

**Universidad Nacional de La Plata** · Teaching · 2016–present

Teaching assistant in two general chemistry courses since 2016, and Chief of Practical Classes for the last year. The content is introductory, so most of the work goes into teaching resources. Knowing the subject in more depth, at a lower level of abstraction, helps the way I explain things.

---

##### Projects

**Lensing** · A framework for building datasets from Qdrant, training and comparing prediction models across Rust, Python and Julia, and exporting selected models to ONNX. An Axum service and a React interface keep experiments, metrics and the model registry together. It follows a modelling loop: propose a model, train and test it, find its shortcomings, and use them for the next iteration.
repo · slides

↳ **Infinite Playlist** · An endless playlist generated from a song or a Spotify playlist. A next-track model trained in Lensing runs in the browser and picks each new song from my listening library.
live demo · repo

**lvv** · A library for converting arbitrary structured data into vector embeddings. It supports configurable transformations, including LLM-assisted preprocessing, before the data enters an embedding or retrieval pipeline.
repo

**convert-invert** · A Rust engine that finds a matching audio file for each track in a playlist. A pool of workers searches, a judge step picks the right file among the candidates, and the state lives in Postgres and Redis. Runs are traced with OpenTelemetry in Jaeger, and a log analyzer compares each run against the tracked reliability issues, reporting retries, peer failures, stalled tasks and duplicate downloads. It runs on a home server.
repo

More on GitHub.

---

##### Research

My PhD in chemistry at UNLP (INIFTA) is on the design, development and characterization of nanosized metal-organic materials for photodynamic therapy. I studied how their structure relates to their photophysical and photochemical processes and dynamics, mainly through spectroscopy. The experimental work and most of the analysis are done, and I'm now writing the thesis. The work was funded by a CONICET doctoral fellowship at INTECH (2019–2024). Before that I completed a licentiate in biotechnology and molecular biology (2013–2018).

In 2023 I co-directed an undergraduate thesis on zirconium- and porphyrin-based materials for the photodegradation of emerging contaminants.

**Publications** · full list on Google Scholar

- 2026 · Development of Metal-Organic Nanoparticles Based on Zr₆(μ₃-O)₄(μ₃-OH)₄ Clusters linked by Perylene-3,4,9,10-Tetracarboxylic Acid · ACS Catalysis
- 2026 · Excited-State Dynamics and Enhanced Singlet Oxygen Photosensitization in a Hf–TCPP Hybrid Material with Nanometer-Scale Structural Features · ACS Applied Nano Materials · first author, submitted
- 2022 · Impact of protein conformational diversity on AlphaFold predictions · Bioinformatics
- 2022 · Incorporation of N and O into the Shell of Silicon Nanoparticles Offers Tunable Photoluminescence for Imaging Uses · ACS Applied Nano Materials
- 2021 · Environmentally Induced Changes of Commercial Carbon Nanotubes in Aqueous Suspensions · ACS Omega
- 2021 · Optimal Silicon-based nanomaterials for biological applications · Concepts and Design of Materials Nanoarchitectonics (book chapter)
- 2020 · Staphylococcus aureus biofilm eradication by the synergistic effect exerted by PEG-coated silicon dots immobilized in silica films and light irradiation · Nanotechnology · first author
- 2020 · Current landscape of the spectroscopic and photochemical properties of β-carboline alkaloids in aqueous media · Journal of the Argentine Chemical Society (review)

**Software from the lab**

- **Glotaran converters** (Rust) · convert time-resolved fluorescence and laser flash photolysis measurements into datasets Glotaran can open.
- **Agilent IR parsers** · read spectra exported by an Agilent Cary 660 ATR-IR and rebuild the missing wavenumber axis.
- **Bilinear calibration** (Rust) · an implementation of a published bilinear calibration method for chemical analysis.
- **xrd_match** · compares measured X-ray diffraction patterns against structures from a CIF database.
- **MOFSocialNet.jl** · a Julia reproduction of the MOFSocialNet method (Jalali et al.) for graph-based analysis of metal-organic frameworks.

---

Eduardo Gonik · 2026

#### Español (borrador)


**Eduardo Gonik**
Ingeniero de datos · La Plata, Argentina
correo · GitHub · LinkedIn · Google Scholar · CV (PDF)

Soy ingeniero de datos. La mayor parte de mi trabajo es desarrollo alrededor de datos (analítica, modelado matemático y testing), y buena parte consiste en construir herramientas para sondear y perfilar sistemas y los datos que manejan. Cuando verifico un sistema, parto de la lógica de lo que tiene que cumplirse y construyo hacia arriba, de la API a la interfaz. Mi formación es en investigación científica y sigo trabajando de esa forma: propongo un modelo, lo pruebo, encuentro dónde falla y uso eso para la siguiente iteración. También doy clases en dos cursos de Química General en la UNLP, y conocer la materia con más profundidad, a un nivel de abstracción más bajo que el del curso, me ayuda a explicarla.

---

##### Trabajo

**Snappler** · Ingeniero de datos · 2025–2026

En el AI Lab de Snappler construí pipelines de análisis de datos asincrónicos y en tiempo real, sistemas agénticos, tracing para aplicaciones de IA y búsqueda RAG.

Para el refactor de un cliente armé la verificación por capas: primero chequear la lógica del cambio desde primeros principios, después una sonda que corre las dos versiones de la API en paralelo sobre los mismos datos, y después una sonda para la interfaz. Las dos sondas también perfilan rendimiento y datos.

**Goodvibes Movement** · Ingeniero de software y datos · 2021–2026

Goodvibes parte de conocimiento científico sobre los ingredientes de los productos. Tiene un sitio que da veredictos sobre productos de consumo y una app en la que fotografiás un producto y obtenés información sobre sus ingredientes y sus riesgos. Fui el único desarrollador interno durante cinco años.

Me ocupé de los datos de ingredientes y productos: buscaba inconsistencias y proponía e implementaba correcciones. Construí la API que lee los ingredientes en fotos de etiquetas. El OCR extrae el texto y cada ingrediente se compara con nuestra base de datos, primero por la menor distancia de Levenshtein y después con una base de datos vectorial. También reescribí el pipeline de reportes: reemplacé un script de R anterior por una herramienta que procesa las bases de ingredientes y genera los reportes XML que consume el sitio, incluidos los puntajes de los productos.

Rust · Axum · Leptos · Julia · Python · Flask · Docker · GCP

**Universidad Nacional de La Plata** · Docencia · 2016–actualidad

Ayudante en dos cursos de Química General desde 2016, y Jefe de Trabajos Prácticos durante el último año. El contenido es introductorio, así que la mayor parte del trabajo está en los recursos didácticos. Conocer la materia con más profundidad, a un nivel de abstracción más bajo, me ayuda a explicar.

---

##### Proyectos

**Lensing** · Un framework para construir datasets a partir de Qdrant, entrenar y comparar modelos de predicción en Rust, Python y Julia, y exportar los modelos elegidos a ONNX. Un servicio en Axum y una interfaz en React mantienen juntos los experimentos, las métricas y el registro de modelos. Sigue un ciclo de modelado: proponer un modelo, entrenarlo y evaluarlo, encontrar sus limitaciones y usarlas para la siguiente iteración.
repo · presentación

↳ **Infinite Playlist** · Una playlist infinita generada a partir de una canción o de una playlist de Spotify. Un modelo de próxima canción entrenado en Lensing corre en el navegador y elige cada tema nuevo de mi biblioteca de música.
demo · repo

**lvv** · Una biblioteca para convertir datos estructurados arbitrarios en embeddings vectoriales. Admite transformaciones configurables, incluido preprocesamiento asistido por LLM, antes de que los datos entren a un pipeline de embeddings o de recuperación.
repo

**convert-invert** · Un motor en Rust que encuentra un archivo de audio para cada tema de una playlist. Un pool de workers busca, un paso de juez elige el archivo correcto entre los candidatos, y el estado vive en Postgres y Redis. Las corridas se trazan con OpenTelemetry en Jaeger, y un analizador de logs compara cada corrida con los problemas de confiabilidad registrados e informa reintentos, fallas de peers, tareas trabadas y descargas duplicadas. Corre en un servidor casero.
repo

Más en GitHub.

---

##### Investigación

Mi doctorado en Química en la UNLP (INIFTA) trata sobre el diseño, desarrollo y caracterización de materiales metal-orgánicos de tamaño nanométrico para terapia fotodinámica. Estudié cómo su estructura se relaciona con sus procesos fotofísicos y fotoquímicos y con su dinámica, principalmente mediante espectroscopía. El trabajo experimental y la mayor parte del análisis están terminados, y ahora estoy escribiendo la tesis. Lo financió una beca doctoral de CONICET en INTECH (2019–2024). Antes me recibí de Licenciado en Biotecnología y Biología Molecular (2013–2018).

En 2023 codirigí una tesina de grado sobre materiales basados en zirconio y porfirinas para la fotodegradación de contaminantes emergentes.

**Publicaciones** · lista completa en Google Scholar

(Los títulos quedan en el idioma original.)

- 2026 · Development of Metal-Organic Nanoparticles Based on Zr₆(μ₃-O)₄(μ₃-OH)₄ Clusters linked by Perylene-3,4,9,10-Tetracarboxylic Acid · ACS Catalysis
- 2026 · Excited-State Dynamics and Enhanced Singlet Oxygen Photosensitization in a Hf–TCPP Hybrid Material with Nanometer-Scale Structural Features · ACS Applied Nano Materials · primer autor, enviado
- 2022 · Impact of protein conformational diversity on AlphaFold predictions · Bioinformatics
- 2022 · Incorporation of N and O into the Shell of Silicon Nanoparticles Offers Tunable Photoluminescence for Imaging Uses · ACS Applied Nano Materials
- 2021 · Environmentally Induced Changes of Commercial Carbon Nanotubes in Aqueous Suspensions · ACS Omega
- 2021 · Optimal Silicon-based nanomaterials for biological applications · Concepts and Design of Materials Nanoarchitectonics (capítulo de libro)
- 2020 · Staphylococcus aureus biofilm eradication by the synergistic effect exerted by PEG-coated silicon dots immobilized in silica films and light irradiation · Nanotechnology · primer autor
- 2020 · Current landscape of the spectroscopic and photochemical properties of β-carboline alkaloids in aqueous media · Journal of the Argentine Chemical Society (revisión)

**Software del laboratorio**

- **Conversores para Glotaran** (Rust) · convierten mediciones de fluorescencia resuelta en el tiempo y de fotólisis de destello láser en datasets que Glotaran puede abrir.
- **Parsers de IR de Agilent** · leen espectros exportados por un ATR-IR Agilent Cary 660 y reconstruyen el eje de número de onda faltante.
- **Calibración bilineal** (Rust) · una implementación de un método publicado de calibración bilineal para análisis químico.
- **xrd_match** · compara patrones de difracción de rayos X medidos con estructuras de una base de datos CIF.
- **MOFSocialNet.jl** · una reproducción en Julia del método MOFSocialNet (Jalali et al.) para el análisis basado en grafos de estructuras metal-orgánicas.

---

Eduardo Gonik · 2026

## Nodos afectados

> Limitación: el MCP de Graphify no conectó en la sesión y el repo no tiene `graphify-out/` (ni
> `GRAPH_REPORT.md` de fallback). Esta sección y la siguiente salen de leer el repo directamente,
> no del subgrafo. Regenerar con /opsx:explore cuando exista el grafo.

El change reemplaza la app completa; casi todos los nodos se eliminan y no se modifican.

| Área | Qué es hoy | Destino en este change |
|---|---|---|
| App Leptos (shell + router) | SSR con Axum e hidratación WASM; dos rutas, `/` y `/es` (`StaticSegment("")`, `StaticSegment("es")`) | se elimina; Zola sirve `/` y `/es/` |
| Componentes de página | `hero`, `nav`, `skills`, `projects` (vistas por capa y por pregunta), `research`, `contact` | se eliminan; un template Zola con la disposición B |
| Datos de contenido | `src/data/projects.rs`: 28 proyectos bilingües (`Text { en, es }`), enums `Layer`, `Question`, `Tick` | se eliminan; el contenido pasa al texto acordado (Markdown/datos de Zola) |
| Estilos | `style/main.scss` (tokens OKLCH, acento naranja `--signal`, fondo crema) + 3 fuentes woff2 propias | se eliminan; CSS mínimo con fuentes del sistema |
| JS de cliente | `public/js/hero-lens.js` (canvas), `public/js/interactions.js` (reveals, nav) | se eliminan; la página no requiere JS |
| JSON legados | `public/assets/data/{projects,works}{,.es}.json` | se eliminan (decisión de la entrevista) |
| Subsitio deck | `public/lensing-presentation/` (HTML/CSS/JS propios, rutas relativas) | se conserva tal cual, movido al directorio estático de Zola |
| Subsitio snappec-verify | `public/snappec-verify/` (página `noindex`, `install.sh`, tarballs `latest/` y `versions/0.1.0/`) | se conserva tal cual, movido al directorio estático de Zola |
| Publicación de snappec-verify | `scripts/publish-snappec-verify.sh` escribe por defecto en `public/snappec-verify` | hay que actualizar su ruta de salida |
| Deploy | workflow de Pages con `cargo-leptos` + servidor levantado para curlear `/` y `/es`; `Dockerfile` y `render.yaml` para el servidor SSR | el workflow pasa a `zola build`; Docker/Render quedan sin función |
| Docs | `README.md` (describe Leptos y el concepto "Lensing field"), `PRODUCT.md`, `DESIGN.md` | README se reescribe; PRODUCT/DESIGN se eliminan |

## Archivos relevantes

> Derivado de leer el repo (ver limitación en "Nodos afectados"), no del subgrafo.

| Archivo | Rol |
|---|---|
| `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml` | build de Leptos/Axum; se eliminan |
| `src/main.rs`, `src/lib.rs`, `src/app.rs` | servidor, hidratación, shell y rutas `/` y `/es`; se eliminan |
| `src/components/{hero,nav,skills,projects,research,contact}.rs` | secciones de la página actual; se eliminan |
| `src/data/projects.rs` | contenido de proyectos (28) y taxonomías; se elimina |
| `style/main.scss` | sistema visual actual; se elimina |
| `public/js/hero-lens.js`, `public/js/interactions.js` | canvas y animaciones; se eliminan |
| `public/fonts/{bricolage,hanken,spline-mono}.woff2` | fuentes propias; se eliminan |
| `public/assets/data/projects.json`, `projects.es.json`, `works.json`, `works.es.json` | catálogo legado; se eliminan |
| `public/favicon.svg` | ícono con gradiente dorado del concepto "Lensing field"; revisar si se reemplaza |
| `public/robots.txt` | hoy hace `Disallow: /lensing-presentation/`; se conserva (ver riesgos) |
| `public/lensing-presentation/**` | deck de Lensing; se conserva sin cambios |
| `public/snappec-verify/**` | página de descarga sin enlazar, `install.sh` y binarios; se conserva sin cambios |
| `scripts/publish-snappec-verify.sh` | genera `public/snappec-verify/`; cambia la ruta de salida |
| `.github/workflows/gh-pages-deploy.yml` | build y deploy a Pages; se reescribe para Zola |
| `Dockerfile`, `.dockerignore`, `render.yaml` | contenedor del servidor SSR; quedan sin función |
| `README.md` | documentación del repo; se reescribe |
| `PRODUCT.md`, `DESIGN.md` | brief y sistema visual anteriores; se eliminan |
| `~/Documents/cv-edu-9-26.pdf` (fuera del repo) | el CV que se publica en `/cv.pdf` |
| `~/Documents/git/project-cv` (fuera del repo) | genera el CV desde un databank bilingüe; no es fuente de este change (non-goal) |
| Nuevos (Zola) | `config.toml` (idiomas en/es), `templates/`, `content/` o `data/` con el texto acordado, `static/` (deck, snappec-verify, `cv.pdf`, imágenes), CSS mínimo |

## Contexto knowledge

> El repo no tiene `knowledge/` (ni `INDEX.md`, `decisions.md` ni use-cases). La intención humana
> previa está documentada en `PRODUCT.md`, `DESIGN.md` y `README.md`, resumida acá.

- **Brief anterior (`PRODUCT.md`):** público = hiring managers y colaboradores; premisa "the site is
  the artifact being judged, so craft is the message"; concepto visual "Lensing field" (lente
  gravitacional como identidad); voz "precise · experimental · quietly confident". Este change
  invierte esa premisa: el sitio deja de ser la pieza de oficio y pasa a ser un perfil sobrio
  escrito con las palabras del usuario.
- **Sistema visual anterior (`DESIGN.md`):** describe un acento dorado sobre papel frío, pero
  `style/main.scss` ya había derivado a fondo crema con acento naranja-óxido (`--signal: oklch(0.69
  0.18 47)`). El usuario lo leyó como "rust" en los dos sentidos (color y lenguaje).
- **README:** documenta el stack Leptos/Axum/cargo-leptos, el render estático en CI de `/` y `/es`,
  y que los JSON legados seguían publicados en `/assets/data/`.
- **Historial reciente:** commits de PRs desde ramas `agent/...` (rediseño "field notes", índice
  por capas y preguntas, ruta `/es`), y las últimas tres sobre la página sin enlazar de
  snappec-verify.
- **Fuente del CV:** `project-cv` (cv-data) guarda registros bilingües validados con procedencia,
  vistas (Industry, Academic, Teaching, One page) y filtrado de datos privados; su README dice que
  se diseñó para alimentar este sitio. Quedó como "tal vez" para más adelante.

## Riesgos identificados

> Sin `knowledge/decisions.md` contra qué chequear; los riesgos salen de leer el repo.

- **Contradicción con el brief documentado:** `PRODUCT.md` y `DESIGN.md` fijan lo opuesto a este
  change ("craft is the message", canvas, animaciones, concepto Lensing). Se resuelve eliminándolos,
  como pide el alcance; si quedaran, el próximo cambio podría volver a esa dirección.
- **Rutas que no se pueden romper:** `install.sh` apunta a
  `https://egonik-unlp.github.io/snappec-verify` (y `.../latest/`, `.../versions/<v>/`); el deck usa
  rutas relativas bajo `/lensing-presentation/`. Mover ambos al directorio estático de Zola tiene que
  preservar las URLs byte a byte.
- **Script de publicación:** `scripts/publish-snappec-verify.sh` escribe por defecto en
  `public/snappec-verify`; si la ruta estática cambia y el script no, la próxima versión se publica
  en un lugar que Zola no sirve.
- **`/es` vs `/es/`:** Zola genera `/es/index.html`. GitHub Pages resuelve `/es` a `/es/`, pero los
  links existentes y el selector de idioma tienen que apuntar a la forma con barra.
- **robots.txt vs link público al deck:** hoy `Disallow: /lensing-presentation/`, y la entrada de
  Lensing enlaza "slides". El link lo hace accesible desde la página aunque no se indexe; decidir si
  se mantiene el `Disallow`.
- **snappec-verify es de Snappler:** el módulo es `gitlab.snappler.com/ai-lab/...`. Se conserva como
  está (página `noindex`, sin enlazar); el change no debe enlazarla desde el perfil.
- **Archivos huérfanos:** `Dockerfile`, `.dockerignore` y `render.yaml` sirven solo al servidor SSR.
  El brief no los menciona; quedan sin función si no se eliminan.
- **Consumidores de los JSON legados:** el README de `project-cv` enlaza
  `/assets/data/projects.json`; al eliminarlo ese link queda roto (su copia en
  `sources/07-projects.json` no se ve afectada).
- **Contenido pendiente del usuario:** correcciones del español; foto y capturas que no se puedan
  obtener; chequeo manual de publicaciones contra Scholar (bloquea acceso automático); color de
  acento asumido.
- **Percepción de convert-invert:** el README muestra workers que se loguean a una red P2P y
  comparten archivos; un lector puede verlo como herramienta de descarga de música. Aceptado por el
  usuario; el texto habla solo de la ingeniería.
- **Coherencia CV ↔ sitio:** el PDF dice "A full rewrite of the ingredient recognition app",
  mientras la entrevista ubica la reescritura en la herramienta de reportes; además tiene typos
  ("responsabilities", "consumed an presented", "Express" duplicado). El PDF se publica tal cual
  salvo que el usuario lo regenere.
- **Archivos sin trackear:** `openspec/`, `.claude/`, `.githooks/`, `.mcp.json`, `.snappec*` y los
  scripts de graphify no están commiteados. Si el sitio enlaza su propio repo, decidir qué de esto se
  publica.
