# UI/UX: replace-portfolio-with-profile-page

**Estado:** Aplica

**Justificación si "No aplica":** —

## Componentes y flujos

Una sola página estática por idioma; no hay estados de carga ni de error, ni eventos de cliente. Los
únicos flujos son leer de arriba abajo, seguir links externos, descargar el CV y cambiar de idioma.

| Componente | Cambio | Estados |
|---|---|---|
| Encabezado | nuevo: foto, nombre (h1), "Data engineer · La Plata, Argentina", links (Email, GitHub, LinkedIn, Scholar), botón "CV (PDF)", link de idioma | con foto / sin foto (el bloque de texto ocupa todo el ancho) |
| Perfil | nuevo: un párrafo, ancho máximo ~760px | — |
| Entrada de Work | nuevo: columna izquierda con organización (h3), rol y período; columna derecha con párrafos y, en Goodvibes, línea de tecnologías | — |
| Entrada de proyecto | nuevo: miniatura 168px a la izquierda, nombre (h3), descripción, links ("repo", "slides", "live demo") | con miniatura / sin miniatura (el texto ocupa todo el ancho) |
| Proyecto anidado (Infinite Playlist) | nuevo: dentro de Lensing, miniatura 120px, nombre (h4) con "built with Lensing" | con / sin miniatura |
| Link "More on GitHub" | nuevo: cierra Projects | — |
| Investigación | nuevo: párrafo del doctorado, línea de codirección, dos columnas: Publications (año · título · revista · marcas) y Software from the lab (nombre enlazado · descripción) | dos columnas en escritorio / una en teléfono |
| Stack (agregado en apply) | nuevo, entre el perfil y Work, siempre visible: `h2` "Stack" como las otras secciones; una fila por grupo con su nombre y etiquetas (rellenas verdes para uso regular, con borde gris para también usado) y una leyenda, entre filetes | — |
| Selector de idioma | nuevo: "Español" en `/`, "English" en `/es/`; lleva a la misma página en el otro idioma | — |
| Footer | nuevo: "Eduardo Gonik · 2026" | — |
| Hero con canvas, nav fija, vistas por capa/pregunta, sección Skills, "Let's talk" | eliminados | — |

Flujos:

1. **Recruiter desde el CV:** abre `/`, lee encabezado y perfil, recorre Work → descarga el CV o
   abre LinkedIn.
2. **Líder técnico:** abre `/`, va a Projects, sigue "repo" o "live demo"; en Research abre un
   repo de software del laboratorio o Scholar.
3. **Visitante hispanohablante:** sigue "Español" → `/es/`, mismo contenido; "English" vuelve a `/`.

## Wireframes / mockups

Mockup de referencia: panel **"B · CV order, photo and thumbnails"** en el canvas
https://claude.ai/artifact/YXcLwxT5bYWAN7hRjzuHtk (privado del usuario). Usar títulos **sans**
(la opción `headingFont: sans` del panel). Las miniaturas y la foto del mockup son placeholders: en
producción, si falta una imagen, no se muestra nada en su lugar.

Escritorio (~960px de contenido):

```
┌──────┐  Eduardo Gonik
│ foto │  Data engineer · La Plata, Argentina
└──────┘  Email  GitHub  LinkedIn  Scholar  [ CV (PDF) ]  Español

Párrafo de perfil (≤760px)

Work
─────────────────────────────────────────────────────────────
Snappler                  │ párrafo AI Lab
Data engineer · 2025–2026 │ párrafo verificación por capas
─────────────────────────────────────────────────────────────
Goodvibes Movement        │ párrafo qué es Goodvibes
Software and data eng. …  │ párrafo datos / API / reportes
                          │ Rust · Axum · … · GCP
─────────────────────────────────────────────────────────────
Universidad Nacional …    │ párrafo docencia
Teaching · 2016–present   │

Projects
─────────────────────────────────────────────────────────────
┌────────┐ Lensing
│ thumb  │ descripción …
└────────┘ repo · slides
           ┌─────┐ Infinite Playlist · built with Lensing
           │thumb│ descripción … live demo · repo
           └─────┘
─────────────────────────────────────────────────────────────
┌────────┐ lvv …            (sin imagen: el texto arranca a la izquierda)
─────────────────────────────────────────────────────────────
┌────────┐ convert-invert …
More on GitHub

Research
párrafo doctorado
línea codirección
Publications · Google Scholar        │ Software from the lab
2026  título · revista               │ Glotaran converters (Rust) · …
…                                    │ …

Eduardo Gonik · 2026
```

Teléfono (360px): todo en una columna. El encabezado apila foto, nombre y links (los links hacen
wrap; el botón del CV queda en su propia línea si no entra). En Work, organización/rol/período quedan
arriba de los párrafos. Las miniaturas quedan arriba de su texto. Publications y Software from the
lab se apilan.

## Accesibilidad

| Aspecto | Decisión |
|---|---|
| Roles ARIA | Landmarks nativos: `<header>`, `<main>`, `<section>` por bloque con su `h2`, `<article>` por entrada, `<footer>`. Jerarquía de títulos h1 (nombre) → h2 (Work, Projects, Research) → h3 (entrada) → h4 (Infinite Playlist, subtítulos de Research). Sin roles ARIA agregados donde alcanza el HTML nativo. |
| Navegación por teclado | Solo links nativos (`<a href>`), en orden de lectura; no hay controles custom ni JS. Links genéricos ("repo", "slides", "live demo") llevan nombre accesible con el proyecto (p. ej. `aria-label="Lensing repository"` o texto visualmente oculto) para que se distingan en la lista de links del lector de pantalla. |
| Foco visible | Contorno de 2px en el color de acento con `outline-offset`, en todos los links, incluido el botón del CV. No se quita el foco del navegador sin reemplazarlo. |
| Contraste | Texto sobre blanco: tinta `#1b1d1c`; secundario `#5d635f` (≥ 4.5:1); acento `#1f5f4a` para links y botón (≥ 4.5:1; texto blanco sobre el botón también ≥ 4.5:1). Los links se distinguen por subrayado, no solo por color. |
| Lectores de pantalla | `lang="en"` / `lang="es"` en `<html>`; `hreflang` en el selector de idioma; "CV (PDF)" anuncia el formato; `alt` descriptivo en cada imagen (p. ej. "Infinite Playlist playing a generated queue"); la foto lleva `alt="Eduardo Gonik"`. Títulos de publicaciones con su idioma original (`lang="en"` dentro de `/es/`). |
| Tamaño de toque | Link del CV ≥ 44px de alto; links del encabezado con espaciado suficiente a 360px. |
| Movimiento | No hay animaciones, transiciones ni smooth-scroll. |
