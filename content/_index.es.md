+++
title = "Eduardo Gonik"
description = "Eduardo Gonik, ingeniero de datos en La Plata, Argentina."

[extra]
photo = "img/photo.jpg"
photo_alt = "Eduardo Gonik"
email = "eduardogonik@gmail.com"
github = "https://github.com/egonik-unlp"
linkedin = "https://www.linkedin.com/in/eduardo-gonik-bbb757133/"
scholar = "https://scholar.google.com/citations?user=0CAay5kAAAAJ"

[[extra.work]]
org = "Snappler"
role = "Ingeniero de datos"
period = "2025–2026"
paragraphs = [
  "En el AI Lab de Snappler trabajé del lado de los datos en productos de IA. Mi proyecto principal fue un asistente inmobiliario con el que los clientes de una inmobiliaria hablaban por WhatsApp. Construí su analítica, en tiempo real y por lotes, para que el equipo pudiera ver cómo iban las conversaciones y cuáles terminaban en leads, y agregué tracing para poder inspeccionar cada llamada al modelo. Diseñé su búsqueda sobre las propiedades de la inmobiliaria y el mercado abierto, y la ajusté con experimentos. También armé la evaluación que medía la calidad del agente, para poder cambiar prompts y modelos y saber si el resultado mejoraba.",
  "Hacia el final, la mayor parte de mi trabajo fue verificación. Para el refactor de un cliente armé la verificación por capas: primero chequear la lógica del cambio desde primeros principios, después una sonda que corre las dos versiones de la API en paralelo sobre los mismos datos, y después una sonda para la interfaz. Las dos sondas también perfilan rendimiento y datos. Después convertí ese enfoque en el módulo de verificación del kit de desarrollo guiado por specs de la empresa, para que otros proyectos pudieran chequear sus cambios de la misma manera.",
  "También modelé datos de sensores de equipos industriales para un posible producto de analítica, y di una charla en Snappler que explicaba cómo funciona un modelo de lenguaje, armándolo desde cero.",
]

[[extra.work]]
org = "Goodvibes Movement"
role = "Ingeniero de software y datos"
period = "2021–2026"
paragraphs = [
  "Goodvibes parte de conocimiento científico sobre los ingredientes de los productos. Tiene un sitio que da veredictos sobre productos de consumo y una app en la que fotografiás un producto y obtenés información sobre sus ingredientes y sus riesgos. Fui el único desarrollador interno durante cinco años.",
  "Me ocupé de los datos de ingredientes y productos: buscaba inconsistencias y proponía e implementaba correcciones. Construí la API que lee los ingredientes en fotos de etiquetas. El OCR extrae el texto y cada ingrediente se compara con nuestra base de datos, primero por la menor distancia de Levenshtein y después con una base de datos vectorial. También reescribí el pipeline de reportes: reemplacé un script de R anterior por una herramienta que procesa las bases de ingredientes y genera los reportes XML que consume el sitio, incluidos los puntajes de los productos.",
]
tech = ["Rust", "Axum", "Leptos", "Julia", "Python", "Flask", "Docker", "GCP"]

[[extra.work]]
org = "Universidad Nacional de La Plata"
role = "Docencia"
period = "2016–actualidad"
paragraphs = [
  "Ayudante en dos cursos de Química General desde 2016, y dos veces Jefe de Trabajos Prácticos, la última de marzo de 2025 a febrero de 2026. El contenido es introductorio, así que la mayor parte del trabajo está en los recursos didácticos. Conocer la materia con más profundidad, a un nivel de abstracción más bajo, me ayuda a explicar.",
]

[[extra.projects]]
name = "Lensing"
description = "Un framework para construir datasets a partir de Qdrant, entrenar y comparar modelos de predicción en Rust, Python y Julia, y exportar los modelos elegidos a ONNX. Un servicio en Axum y una interfaz en React mantienen juntos los experimentos, las métricas y el registro de modelos. Sigue un ciclo de modelado: proponer un modelo, entrenarlo y evaluarlo, encontrar sus limitaciones y usarlas para la siguiente iteración."
image = "img/lensing.png"
image_alt = "Una slide de la presentación de Lensing"
links = [
  { label = "repo", url = "https://github.com/egonik-unlp/lensing" },
  { label = "presentación", url = "/lensing-presentation/" },
]

[[extra.projects.children]]
name = "Infinite Playlist"
description = "Una playlist infinita generada a partir de una canción o de una playlist de Spotify. Un modelo de próxima canción entrenado en Lensing corre en el navegador y elige cada tema nuevo de mi biblioteca de música."
image = "img/infinite-playlist.png"
image_alt = "La demo de Infinite Playlist"
links = [
  { label = "demo", url = "https://infinite-playlist.eduardo-gonik.workers.dev" },
  { label = "repo", url = "https://github.com/egonik-unlp/spotify-next-track" },
]

[[extra.projects]]
name = "lvv"
description = "Una biblioteca para convertir datos estructurados arbitrarios en embeddings vectoriales. Admite transformaciones configurables, incluido preprocesamiento asistido por LLM, antes de que los datos entren a un pipeline de embeddings o de recuperación."
image = "img/lvv.png"
image_alt = "El logotipo de lvv: las letras l, v, v en celdas de memoria, con flechas de vector sobre las v"
links = [
  { label = "repo", url = "https://github.com/egonik-unlp/lvv" },
]

[[extra.projects]]
name = "convert-invert"
description = "Un motor en Rust que encuentra un archivo de audio para cada tema de una playlist. Un pool de workers busca, un paso de juez elige el archivo correcto entre los candidatos, y el estado vive en Postgres y Redis. Las corridas se trazan con OpenTelemetry en Jaeger, y un analizador de logs compara cada corrida con los problemas de confiabilidad registrados e informa reintentos, fallas de peers, tareas trabadas y descargas duplicadas. Corre en un servidor casero."
image = "img/convert-invert.png"
image_alt = "Una red de pares con una ruta de búsqueda resaltada hasta el par que tiene el archivo"
links = [
  { label = "repo", url = "https://github.com/egonik-unlp/convert-invert" },
]

[extra.research]
paragraphs = [
  "Mi doctorado en Química en la UNLP (INIFTA) trata sobre el diseño, desarrollo y caracterización de materiales metal-orgánicos de tamaño nanométrico para terapia fotodinámica. Estudié cómo su estructura se relaciona con sus procesos fotofísicos y fotoquímicos y con su dinámica, principalmente mediante espectroscopía. El trabajo experimental y la mayor parte del análisis están terminados, y ahora estoy escribiendo la tesis. Lo financió una beca doctoral de CONICET en INTECH (2019–2024). De octubre a diciembre de 2024 estuve en el Departamento de Química de la Universidad de Turín, con un intercambio Marie Skłodowska-Curie (proyecto PHENOCYCLES). Antes me recibí de Licenciado en Biotecnología y Biología Molecular (2013–2018).",
  "En 2023 codirigí una tesina de grado sobre materiales basados en zirconio y porfirinas para la fotodegradación de contaminantes emergentes.",
]

[[extra.lab_software]]
name = "Conversores para Glotaran"
note = "Rust"
description = "convierten mediciones de fluorescencia resuelta en el tiempo y de fotólisis de destello láser en datasets que Glotaran puede abrir."
url = "https://github.com/egonik-unlp/glotaran_converter_lib"

[[extra.lab_software]]
name = "Parsers de IR de Agilent"
description = "leen espectros exportados por un ATR-IR Agilent Cary 660 y reconstruyen el eje de número de onda faltante."
url = "https://github.com/egonik-unlp/agilent_ir"

[[extra.lab_software]]
name = "Calibración bilineal"
note = "Rust"
description = "una implementación de un método publicado de calibración bilineal para análisis químico."
url = "https://github.com/egonik-unlp/bilinear"

[[extra.lab_software]]
name = "xrd_match"
description = "compara patrones de difracción de rayos X medidos con estructuras de una base de datos CIF."
url = "https://github.com/egonik-unlp/xrd_match"

[[extra.lab_software]]
name = "MOFSocialNet.jl"
description = "una reproducción en Julia del método MOFSocialNet (Jalali et al.) para el análisis basado en grafos de estructuras metal-orgánicas."
url = "https://github.com/egonik-unlp/MOFSocialNet.jl"

[[extra.stack]]
group = "Lenguajes"
regular = ["Rust", "JavaScript / TypeScript", "Python", "Julia"]
also = ["Zig", "Go", "OCaml"]

[[extra.stack]]
group = "ML, IA y ciencia de datos"
regular = ["LangChain", "LangGraph", "Embeddings", "RAG", "PyTorch", "Deep learning", "Modelado matemático"]
also = ["Keras", "Ecosistema de datos de Julia", "burn", "Qdrant", "Chroma", "LlamaIndex", "Flux.jl", "scikit-learn / XGBoost", "ONNX", "Langfuse", "Pipelines ETL"]

[[extra.stack]]
group = "Bases de datos"
regular = ["PostgreSQL"]
also = ["Redis"]

[[extra.stack]]
group = "Sistemas y web"
regular = ["Axum", "Express", "tokio", "Diesel", "Leptos"]
also = ["Actix Web", "Flask", "OpenTelemetry / Jaeger", "Sistemas multi-worker"]

[[extra.stack]]
group = "Infraestructura y plataformas"
regular = ["Linux", "Docker y Compose"]
also = ["Windows", "macOS", "AWS y GCP"]
+++

Soy ingeniero de datos. La mayor parte de mi trabajo es desarrollo alrededor de datos (analítica, modelado matemático y testing), y buena parte consiste en construir herramientas para sondear y perfilar sistemas y los datos que manejan. Cuando verifico un sistema, parto de la lógica de lo que tiene que cumplirse y construyo hacia arriba, de la API a la interfaz. Mi formación es en investigación científica y sigo trabajando de esa forma: propongo un modelo, lo pruebo, encuentro dónde falla y uso eso para la siguiente iteración. También doy clases en dos cursos de Química General en la UNLP, y conocer la materia con más profundidad, a un nivel de abstracción más bajo que el del curso, me ayuda a explicarla.
