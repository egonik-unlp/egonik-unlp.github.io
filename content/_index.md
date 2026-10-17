+++
title = "Eduardo Gonik"
description = "Eduardo Gonik, data engineer in La Plata, Argentina."

[extra]
photo = "img/photo.jpg"
photo_alt = "Eduardo Gonik"
email = "eduardogonik@gmail.com"
github = "https://github.com/egonik-unlp"
linkedin = "https://www.linkedin.com/in/eduardo-gonik-bbb757133/"
scholar = "https://scholar.google.com/citations?user=0CAay5kAAAAJ"

[[extra.work]]
org = "Snappler"
role = "Data engineer"
period = "2025–2026"
paragraphs = [
  "In Snappler's AI Lab I built asynchronous and real-time data analysis pipelines, agentic systems, tracing for AI applications, and RAG search.",
  "For a client's refactor, I built the verification in layers: first checking the logic of the change from first principles, then a probe that runs both versions of the API side by side on the same data, then a probe for the interface. Both probes also profile performance and data.",
]

[[extra.work]]
org = "Goodvibes Movement"
role = "Software and data engineer"
period = "2021–2026"
paragraphs = [
  "Goodvibes builds on scientific knowledge about product ingredients. It runs a site that gives verdicts on consumer products, and an app where you photograph a product and get information about its ingredients and their risks. I was the only internal developer for five years.",
  "I looked after the ingredient and product data, searching for inconsistencies and proposing and implementing fixes. I built the API that reads ingredients from label photos. OCR extracts the text, and each ingredient is matched to our database, at first by the smallest Levenshtein distance and later with a vector database. I also rewrote the reporting pipeline, replacing an older R script with a tool that parses the ingredient databases and generates the XML reports the site consumes, product scores included.",
]
tech = ["Rust", "Axum", "Leptos", "Julia", "Python", "Flask", "Docker", "GCP"]

[[extra.work]]
org = "Universidad Nacional de La Plata"
role = "Teaching"
period = "2016–present"
paragraphs = [
  "Teaching assistant in two general chemistry courses since 2016, and Chief of Practical Classes for the last year. The content is introductory, so most of the work goes into teaching resources. Knowing the subject in more depth, at a lower level of abstraction, helps the way I explain things.",
]

[[extra.projects]]
name = "Lensing"
description = "A framework for building datasets from Qdrant, training and comparing prediction models across Rust, Python and Julia, and exporting selected models to ONNX. An Axum service and a React interface keep experiments, metrics and the model registry together. It follows a modelling loop: propose a model, train and test it, find its shortcomings, and use them for the next iteration."
image = "img/lensing.png"
image_alt = "A slide from the Lensing presentation"
links = [
  { label = "repo", url = "https://github.com/egonik-unlp/lensing" },
  { label = "slides", url = "/lensing-presentation/" },
]

[[extra.projects.children]]
name = "Infinite Playlist"
description = "An endless playlist generated from a song or a Spotify playlist. A next-track model trained in Lensing runs in the browser and picks each new song from my listening library."
image = "img/infinite-playlist.png"
image_alt = "The Infinite Playlist demo"
links = [
  { label = "live demo", url = "https://infinite-playlist.eduardo-gonik.workers.dev" },
  { label = "repo", url = "https://github.com/egonik-unlp/spotify-next-track" },
]

[[extra.projects]]
name = "lvv"
description = "A library for converting arbitrary structured data into vector embeddings. It supports configurable transformations, including LLM-assisted preprocessing, before the data enters an embedding or retrieval pipeline."
image = "img/lvv.png"
image_alt = "The lvv logotype: the letters l, v, v in memory cells, with vector arrows over the v's"
links = [
  { label = "repo", url = "https://github.com/egonik-unlp/lvv" },
]

[[extra.projects]]
name = "convert-invert"
description = "A Rust engine that finds a matching audio file for each track in a playlist. A pool of workers searches, a judge step picks the right file among the candidates, and the state lives in Postgres and Redis. Runs are traced with OpenTelemetry in Jaeger, and a log analyzer compares each run against the tracked reliability issues, reporting retries, peer failures, stalled tasks and duplicate downloads. It runs on a home server."
image = "img/convert-invert.png"
image_alt = "A peer network with one search route highlighted to the peer that has the file"
links = [
  { label = "repo", url = "https://github.com/egonik-unlp/convert-invert" },
]

[extra.research]
paragraphs = [
  "My PhD in chemistry at UNLP (INIFTA) is on the design, development and characterization of nanosized metal-organic materials for photodynamic therapy. I studied how their structure relates to their photophysical and photochemical processes and dynamics, mainly through spectroscopy. The experimental work and most of the analysis are done, and I'm now writing the thesis. The work was funded by a CONICET doctoral fellowship at INTECH (2019–2024). Before that I completed a licentiate in biotechnology and molecular biology (2013–2018).",
  "In 2023 I co-directed an undergraduate thesis on zirconium- and porphyrin-based materials for the photodegradation of emerging contaminants.",
]

[[extra.lab_software]]
name = "Glotaran converters"
note = "Rust"
description = "convert time-resolved fluorescence and laser flash photolysis measurements into datasets Glotaran can open."
url = "https://github.com/egonik-unlp/glotaran_converter_lib"

[[extra.lab_software]]
name = "Agilent IR parsers"
description = "read spectra exported by an Agilent Cary 660 ATR-IR and rebuild the missing wavenumber axis."
url = "https://github.com/egonik-unlp/agilent_ir"

[[extra.lab_software]]
name = "Bilinear calibration"
note = "Rust"
description = "an implementation of a published bilinear calibration method for chemical analysis."
url = "https://github.com/egonik-unlp/bilinear"

[[extra.lab_software]]
name = "xrd_match"
description = "compares measured X-ray diffraction patterns against structures from a CIF database."
url = "https://github.com/egonik-unlp/xrd_match"

[[extra.lab_software]]
name = "MOFSocialNet.jl"
description = "a Julia reproduction of the MOFSocialNet method (Jalali et al.) for graph-based analysis of metal-organic frameworks."
url = "https://github.com/egonik-unlp/MOFSocialNet.jl"

[[extra.stack]]
group = "Languages"
regular = ["Rust", "JavaScript / TypeScript", "Python", "Julia"]
also = ["Zig", "Go", "OCaml"]

[[extra.stack]]
group = "ML & AI"
regular = ["LangChain", "LangGraph", "Embeddings", "RAG", "PyTorch", "Deep learning"]
also = ["Keras", "burn", "Qdrant", "Chroma", "LlamaIndex", "Flux.jl", "scikit-learn / XGBoost", "ONNX", "Langfuse"]

[[extra.stack]]
group = "Data"
regular = ["PostgreSQL", "Mathematical modelling"]
also = ["Redis", "Julia data ecosystem", "ETL pipelines", "pandas / polars", "Job queues", "Web scraping"]

[[extra.stack]]
group = "Systems & Web"
regular = ["Axum", "Express", "tokio", "Diesel"]
also = ["Actix Web", "Leptos", "Flask", "OpenTelemetry / Jaeger", "Multi-worker systems"]

[[extra.stack]]
group = "Infra & Platforms"
regular = ["Linux", "Docker & Compose"]
also = ["Windows", "macOS", "Cloudflare Workers", "Portainer", "AWS & GCP"]
+++

I'm a data engineer. Most of my work is development around data (analytics, mathematical modelling and testing), and a good part of it is building tools to probe and profile systems and the data they handle. When I verify a system, I start from the logic of what has to hold and build upward, from the API to the interface. My background is in scientific research, and I still work that way: I propose a model, test it, find where it falls short, and use that for the next iteration. I also teach two general chemistry courses at UNLP, and knowing the subject in more depth, at a lower level of abstraction than the course, helps me explain it.
