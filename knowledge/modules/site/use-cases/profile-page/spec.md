---
owner: Eduardo Gonik
last_updated: 2026-10-17
status: draft
---

# profile-page — contrato

Contrato formal: [[openspec/specs/profile-page/spec.md]]. Origen: change
`2026-10-17-replace-portfolio-with-profile-page`.

## Propósito

Presentar el perfil profesional de Eduardo Gonik (ingeniería, investigación y docencia) como una página
estática, sobria y bilingüe que recruiters y líderes técnicos puedan leer y verificar, con el CV
descargable, sin romper los subsitios estáticos que ya se publican.

## Qué debe cumplirse

- **Dos idiomas, misma página.** Inglés en `/`, español en `/es/`, con las mismas secciones, entradas y
  links; cada versión enlaza a la otra.
- **Encabezado.** Nombre, "Data engineer · La Plata, Argentina", correo, GitHub, LinkedIn, Google Scholar
  y un botón "CV (PDF)" que descarga `/cv.pdf`.
- **Orden.** Perfil → Stack → Work → Projects → Research.
- **Texto solo del acordado.** Todo lo visible sale del texto registrado en la entrevista del change
  (o, para Stack, de la lista acordada en su design). Nada de taglines, métricas ni afirmaciones nuevas.
- **Stack siempre visible.** Cinco grupos; en cada uno, etiquetas rellenas para lo que se usa
  habitualmente y etiquetas con borde para lo que también se usó, con leyenda; cada tipo es una lista
  rotulada para lectores de pantalla.
- **Work.** Snappler, Goodvibes Movement y la docencia en la UNLP. El cliente del trabajo de verificación
  en Snappler no se nombra.
- **Projects.** Exactamente Lensing (con Infinite Playlist debajo), lvv y convert-invert, con sus links,
  y un "More on GitHub".
- **Research.** Doctorado (tesis en escritura), codirección de tesina sin nombrar a la estudiante, 8
  publicaciones con Google Scholar como lista completa, 5 herramientas de laboratorio con su repositorio.
- **Imágenes opcionales.** Si falta una imagen, la entrada se muestra sin marco ni placeholder.
- **Sin JS ni animación**, sin fuentes web; legible a 360 px sin scroll horizontal y con el link del CV
  de al menos 44 px de alto.
- **Subsitios intactos.** `/lensing-presentation/` y `/snappec-verify/` (con `install.sh`, `latest/` y
  `versions/`) siguen en las mismas URLs; el script de publicación escribe donde el build los sirve.
- **Indexación.** El deck es indexable; la página de snappec-verify sigue `noindex` y sin enlazar.
- **Deploy.** Build estático de Zola publicado en GitHub Pages por el workflow, sin servidor; los JSON
  legados de `/assets/data/` ya no se sirven.
