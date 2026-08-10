//! Portfolio content. Editing the site is mostly editing this file.

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Category {
    AiMl,
    Systems,
    Visual,
}

impl Category {
    pub fn label(&self, spanish: bool) -> &'static str {
        match (self, spanish) {
            (Category::AiMl, false) => "ai & machine learning",
            (Category::Systems, false) => "systems & backend",
            (Category::Visual, false) => "visual & creative",
            (Category::AiMl, true) => "ia y aprendizaje automático",
            (Category::Systems, true) => "sistemas y backend",
            (Category::Visual, true) => "visual y creativo",
        }
    }

    /// Display order for the Projects section.
    pub fn ordered() -> [Category; 3] {
        [Category::AiMl, Category::Systems, Category::Visual]
    }
}

#[derive(Clone, Copy)]
pub struct Link {
    pub label: &'static str,
    pub url: &'static str,
}

#[derive(Clone)]
pub struct Project {
    pub title: &'static str,
    pub tagline: &'static str,
    pub description: &'static str,
    pub tech: &'static [&'static str],
    pub links: &'static [Link],
    pub category: Category,
    /// Repository is private (site is private too, so links still shown).
    pub private: bool,
}

pub fn projects(spanish: bool) -> Vec<Project> {
    let mut projects = vec![
        // ---- AI & Machine Learning ---------------------------------------
        Project {
            title: "Lensing",
            tagline: "A framework for running prediction experiments on different datasets.",
            description: "Lensing builds datasets from Qdrant, trains models in Rust, Python and \
                Julia, compares their results, and exports selected models to ONNX. I use it for \
                real-estate and Spotify experiments. An Axum server and React interface keep the \
                runs, metrics and model registry in one place.",
            tech: &[
                "Rust", "burn", "Python", "PyTorch", "Julia", "Flux.jl", "scikit-learn", "Qdrant",
                "ONNX", "Axum", "React", "Zig",
            ],
            links: &[Link { label: "GitHub", url: "https://github.com/egonik-unlp/lensing" }],
            category: Category::AiMl,
            private: false,
        },
        Project {
            title: "Pathfinder",
            tagline: "A 3D route between two songs in my Spotify listening data.",
            description: "Pathfinder uses a model exported from Lensing to find a sequence between \
                two tracks. The route search runs in Rust compiled to WebAssembly, the visualization \
                uses React and Three.js, and a Cloudflare Worker handles the small backend.",
            tech: &["Rust/WASM", "React", "Three.js", "ONNX", "Cloudflare Workers"],
            links: &[Link { label: "GitHub", url: "https://github.com/egonik-unlp/app" }, Link{label: "Live Site", url: "https://pathfinder.eduardo-gonik.workers.dev/"}],
            category: Category::AiMl,
            private: false,
        },
        Project {
            title: "lvv",
            tagline: "A Rust pipeline for loading datasets into a vector database.",
            description: "lvv reads data from CSV files or Postgres, creates embeddings with Ollama \
                or OpenAI, and writes them to Qdrant. It includes a job queue and a disk cache so \
                interrupted imports do not have to start again.",
            tech: &["Rust", "Qdrant", "OpenAI", "Ollama", "Postgres", "tokio"],
            links: &[Link { label: "GitHub", url: "https://github.com/egonik-unlp/lvv" }],
            category: Category::AiMl,
            private: false,
        },
        Project {
            title: "craig",
            tagline: "A chemistry question-answering experiment using retrieval.",
            description: "craig indexes a physical-chemistry textbook in Qdrant or Chroma and uses \
                the retrieved passages when answering questions. It exposes an HTTP API and a \
                Telegram bot, with Langfuse and OpenTelemetry traces for inspecting responses.",
            tech: &[
                "TypeScript", "LangChain", "LlamaIndex", "Qdrant", "Chroma", "OpenAI", "Langfuse",
            ],
            links: &[Link { label: "GitHub", url: "https://github.com/egonik-unlp/craig" }],
            category: Category::AiMl,
            private: false,
        },
        Project {
            title: "toy-server",
            tagline: "A small HTTP server written directly on top of TCP.",
            description: "A learning project that accepts connections with Rust's TcpListener, \
                parses HTTP request lines and headers, routes paths to handlers, and writes HTTP \
                responses without using a web framework.",
            tech: &["Rust", "TCP", "HTTP"],
            links: &[Link {
                label: "GitHub",
                url: "https://github.com/egonik-unlp/toy-server",
            }],
            category: Category::Systems,
            private: false,
        },
        // ---- Systems & Backend -------------------------------------------
        Project {
            title: "convert-ffi",
            tagline: "A full-stack Rust interface backed by a native Zig library.",
            description: "The browser reads metadata from local audio files, then a Leptos and Axum \
                application sends the track details through Rust FFI to a Zig Spotify client. The \
                user can review the matches and create a playlist. Audio files stay in the browser.",
            tech: &["Rust", "Leptos", "Axum", "Zig", "FFI", "Spotify API", "WebAssembly"],
            links: &[
                Link { label: "GitHub", url: "https://github.com/egonik-unlp/convert-ffi" },
                Link { label: "Live Site", url: "https://convert-ffi.onrender.com" },
            ],
            category: Category::Systems,
            private: false,
        },
        Project {
            title: "convert-songs",
            tagline: "A Zig CLI that turns a folder of music into a Spotify playlist.",
            description: "convert-songs reads ID3 tags, searches Spotify for each track, and creates \
                a playlist from the matches. The OAuth flow, token handling and API client are \
                implemented directly in Zig.",
            tech: &["Zig", "OAuth2", "Spotify API", "ID3"],
            links: &[Link { label: "GitHub", url: "https://github.com/egonik-unlp/convert-songs" }],

            category: Category::Systems,
            private: false,
        },
        Project {
            title: "Infinite Playlist",
            tagline: "An endless playlist generated from a song or Spotify playlist.",
            description: "Infinite Playlist is a client for the spotify-next-track Lensing project. \
                A GRU model picks each next song from my library and runs in the browser through \
                ONNX Runtime. A Cloudflare Worker resolves Spotify links and handles tracks that \
                are not already in the model's catalog.",
            tech: &["Python", "PyTorch", "ONNX", "Rust/WASM", "JavaScript", "Cloudflare Workers"],
            links: &[Link {
                label: "Live Site",
                url: "https://infinite-playlist.eduardo-gonik.workers.dev/",
            }],
            category: Category::AiMl,
            private: false,
        },
        Project {
            title: "egonik-site",
            tagline: "A database-backed personal site written in Rust.",
            description: "A full-stack Leptos application with server rendering and client-side \
                hydration. Actix serves the site, Diesel reads portfolio and publication data from \
                Postgres, and the same Rust components are used on the server and in WebAssembly.",
            tech: &["Rust", "Leptos", "Actix", "Diesel", "Postgres", "WebAssembly", "Docker"],
            links: &[
                Link { label: "GitHub", url: "https://github.com/egonik-unlp/egonik-site" },
                Link {
                    label: "Live Site",
                    url: "https://site-production-20aa.up.railway.app/#code",
                },
            ],
            category: Category::Systems,
            private: false,
        },
        // ---- Visual & Creative -------------------------------------------
        Project {
            title: "flagen",
            tagline: "A small generative-art program based on random walks.",
            description: "flagen uses parallel random walkers and a Monte Carlo process to build \
                images and height maps. It is written in OCaml with the Owl numerical library and \
                exports both rendered images and raw matrices.",
            tech: &["OCaml", "Owl", "Monte Carlo", "Python"],
            links: &[Link { label: "GitHub", url: "https://github.com/egonik-unlp/flagen" }],
            category: Category::Visual,
            private: false,
        },
    ];

    if spanish {
        let translations = [
            (
                "Un framework para ejecutar experimentos de predicción sobre distintos conjuntos de datos.",
                "Lensing construye datasets desde Qdrant, entrena modelos en Rust, Python y Julia, compara sus resultados y exporta los modelos seleccionados a ONNX. Lo uso en experimentos inmobiliarios y de Spotify. Un servidor Axum y una interfaz React reúnen las ejecuciones, métricas y el registro de modelos.",
            ),
            (
                "Una ruta 3D entre dos canciones de mi historial de Spotify.",
                "Pathfinder usa un modelo exportado desde Lensing para encontrar una secuencia entre dos canciones. La búsqueda de rutas corre en Rust compilado a WebAssembly, la visualización usa React y Three.js, y un Cloudflare Worker se ocupa del pequeño backend.",
            ),
            (
                "Un pipeline en Rust para cargar datasets en una base de datos vectorial.",
                "lvv lee datos desde archivos CSV o Postgres, crea embeddings con Ollama u OpenAI y los escribe en Qdrant. Incluye una cola de trabajos y caché en disco para que las importaciones interrumpidas no tengan que empezar de nuevo.",
            ),
            (
                "Un experimento de preguntas y respuestas de química basado en recuperación.",
                "craig indexa un libro de fisicoquímica en Qdrant o Chroma y usa los pasajes recuperados para responder preguntas. Expone una API HTTP y un bot de Telegram, con trazas de Langfuse y OpenTelemetry para inspeccionar las respuestas.",
            ),
            (
                "Un pequeño servidor HTTP escrito directamente sobre TCP.",
                "Un proyecto de aprendizaje que acepta conexiones con TcpListener de Rust, interpreta líneas y cabeceras HTTP, enruta paths a handlers y escribe respuestas HTTP sin usar un framework web.",
            ),
            (
                "Una interfaz full-stack en Rust respaldada por una biblioteca nativa en Zig.",
                "El navegador lee los metadatos de archivos de audio locales y una aplicación Leptos y Axum envía los datos de las canciones, mediante FFI de Rust, a un cliente de Spotify en Zig. La persona puede revisar las coincidencias y crear una playlist. Los archivos de audio permanecen en el navegador.",
            ),
            (
                "Una CLI en Zig que convierte una carpeta de música en una playlist de Spotify.",
                "convert-songs lee etiquetas ID3, busca cada canción en Spotify y crea una playlist con las coincidencias. El flujo OAuth, la gestión de tokens y el cliente de la API están implementados directamente en Zig.",
            ),
            (
                "Una playlist infinita generada desde una canción o playlist de Spotify.",
                "Infinite Playlist es un cliente para el proyecto Lensing spotify-next-track. Un modelo GRU elige cada canción siguiente desde mi biblioteca y corre en el navegador mediante ONNX Runtime. Un Cloudflare Worker resuelve enlaces de Spotify y gestiona canciones que todavía no están en el catálogo del modelo.",
            ),
            (
                "Un sitio personal con base de datos escrito en Rust.",
                "Una aplicación full-stack en Leptos con renderizado en servidor e hidratación en el cliente. Actix sirve el sitio, Diesel lee el portfolio y las publicaciones desde Postgres, y los mismos componentes en Rust se usan en el servidor y en WebAssembly.",
            ),
            (
                "Un pequeño programa de arte generativo basado en caminos aleatorios.",
                "flagen usa caminantes aleatorios en paralelo y un proceso de Monte Carlo para construir imágenes y mapas de altura. Está escrito en OCaml con la biblioteca numérica Owl y exporta tanto imágenes renderizadas como matrices crudas.",
            ),
        ];

        for (project, (tagline, description)) in projects.iter_mut().zip(translations) {
            project.tagline = tagline;
            project.description = description;
        }
    }

    projects
}
