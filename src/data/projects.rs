//! Portfolio content. Editing the site is mostly editing this file.
//!
//! One index, two readings. Every project declares the [`Layer`] it sits in —
//! how far from the screen it lives — and, if it earns one, the [`Question`] it
//! set out to answer. A project without a question is not hidden: it falls into
//! the closing group at the end of the question view.

/// A string in both site languages. Explicit pairs rather than a positional
/// translation table, so adding a project cannot silently shift the Spanish.
#[derive(Clone, Copy)]
pub struct Text {
    pub en: &'static str,
    pub es: &'static str,
}

impl Text {
    pub fn get(&self, spanish: bool) -> &'static str {
        if spanish {
            self.es
        } else {
            self.en
        }
    }
}

const fn t(en: &'static str, es: &'static str) -> Text {
    Text { en, es }
}

/// How far from the screen a project sits. The label *is* the explanation —
/// deliberately no glossary, no diagram, no jargon.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Layer {
    See,
    Use,
    Works,
    Knows,
    Runs,
}

impl Layer {
    pub fn label(&self, spanish: bool) -> &'static str {
        match self {
            Layer::See => t("what you see", "lo que se ve"),
            Layer::Use => t("what you use", "lo que se usa"),
            Layer::Works => t("what it works out", "lo que calcula"),
            Layer::Knows => t("what it knows", "lo que sabe"),
            Layer::Runs => t("what it runs on", "sobre lo que corre"),
        }
        .get(spanish)
    }

    /// Screen first, machine last.
    pub fn ordered() -> [Layer; 5] {
        [
            Layer::See,
            Layer::Use,
            Layer::Works,
            Layer::Knows,
            Layer::Runs,
        ]
    }
}

/// The second reading: what the work was trying to find out. Ordered so the
/// questions carrying the most interesting projects come first.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Question {
    HearNext,
    Shelf,
    Instrument,
    Collection,
    PickWord,
    Toolkit,
    Material,
    FromPaper,
    Textbook,
    MachineDraws,
}

impl Question {
    pub fn ordered() -> [Question; 10] {
        [
            Question::HearNext,
            Question::Shelf,
            Question::Instrument,
            Question::Collection,
            Question::PickWord,
            Question::Toolkit,
            Question::Material,
            Question::FromPaper,
            Question::Textbook,
            Question::MachineDraws,
        ]
    }

    /// Label for the group collecting projects with no question of their own.
    pub fn rest_label(spanish: bool) -> &'static str {
        t("the rest of the shelf", "el resto del estante").get(spanish)
    }

    pub fn text(&self, spanish: bool) -> &'static str {
        match self {
            Question::HearNext => t(
                "Can a computer guess what you'll want to hear next?",
                "¿Puede una computadora adivinar qué vas a querer escuchar después?",
            ),
            Question::Shelf => t(
                "What is actually in the products on the shelf?",
                "¿Qué hay realmente en los productos de la góndola?",
            ),
            Question::Instrument => t(
                "Can a lab instrument's output become something anyone can read?",
                "¿Puede la salida de un instrumento de laboratorio volverse legible para \
                 cualquiera?",
            ),
            Question::Collection => t(
                "What does it take to keep a music collection that outlives the service?",
                "¿Qué hace falta para conservar una colección de música que sobreviva al servicio?",
            ),
            Question::PickWord => t(
                "What is a language model doing when it picks a word?",
                "¿Qué hace un modelo de lenguaje cuando elige una palabra?",
            ),
            Question::Toolkit => t(
                "Can one toolkit be pointed at any prediction problem?",
                "¿Se puede apuntar una misma caja de herramientas a cualquier problema de \
                 predicción?",
            ),
            Question::Material => t(
                "What material am I actually holding?",
                "¿Qué material tengo realmente en la mano?",
            ),
            Question::FromPaper => t(
                "Can a method from a scientific paper become software other people run?",
                "¿Puede un método publicado convertirse en software que otras personas ejecuten?",
            ),
            Question::Textbook => t(
                "Can a computer answer questions about a real textbook?",
                "¿Puede una computadora responder preguntas sobre un libro de texto real?",
            ),
            Question::MachineDraws => t(
                "What comes out if you let the machine draw?",
                "¿Qué sale si se deja dibujar a la máquina?",
            ),
        }
        .get(spanish)
    }

    pub fn answer(&self, spanish: bool) -> &'static str {
        match self {
            Question::HearNext => t(
                "Partly — and only by combining three unlike methods. A neural model on its own \
                 ties a simple tally of what usually follows what; add a \"sounds similar\" pass \
                 and they land the real next song in the top ten about 21% of the time, 61% better \
                 than the tally alone. Making the network cleverer never once helped.",
                "En parte, y solo combinando tres métodos distintos. Un modelo neuronal por sí \
                 solo empata con un simple conteo de qué canción suele seguir a cuál; al sumarle \
                 una pasada de \"suena parecido\", aciertan la canción siguiente entre las diez \
                 primeras cerca del 21% de las veces, un 61% mejor que el conteo solo. Hacer la \
                 red más compleja no ayudó nunca.",
            ),
            Question::Shelf => t(
                "Answerable, but only by building the whole chain: photograph the label, read the \
                 text off it, match the printed ingredients against a curated database, report by \
                 category. The first project big enough to need a dozen repositories, back in 2021.",
                "Se puede responder, pero solo construyendo toda la cadena: fotografiar la \
                 etiqueta, leer el texto, comparar los ingredientes impresos contra una base de \
                 datos curada y reportar por categoría. El primer proyecto lo bastante grande como \
                 para necesitar una docena de repositorios, allá en 2021.",
            ),
            Question::Instrument => t(
                "That is what most of five years of lab code was for. Every machine writes its own \
                 private format and the knowledge of how to read it lives in one person's head; \
                 these move it into software, so a measurement can be re-analysed years later by \
                 someone else.",
                "Para eso sirvió la mayor parte de cinco años de código de laboratorio. Cada \
                 equipo escribe su propio formato privado y el conocimiento de cómo leerlo vive en \
                 la cabeza de una sola persona; esto lo pasa a software, así una medición puede \
                 reanalizarse años después y por otra persona.",
            ),
            Question::Collection => t(
                "More than a script. A pool of workers, a judge deciding which of several \
                 candidate files is really the track you asked for, retries, and tracing to see \
                 where things stall. It has run unattended on a home server ever since.",
                "Más que un script. Un pool de workers, un juez que decide cuál de varios archivos \
                 candidatos es realmente la canción pedida, reintentos y trazas para ver dónde se \
                 traba. Desde entonces corre sin supervisión en un servidor casero.",
            ),
            Question::PickWord => t(
                "Choosing between a handful of candidates, over and over. Obvious the moment you \
                 animate it, invisible written down — which is the whole argument for building the \
                 demo.",
                "Elegir entre un puñado de candidatas, una y otra vez. Evidente en cuanto se \
                 anima, invisible por escrito: justamente el argumento para construir la demo.",
            ),
            Question::Toolkit => t(
                "Yes, if the toolkit rebuilds itself. Describe the new data in one file and the \
                 controls, the competing models, the dashboard and the exports all take its shape. \
                 Five projects started this way; one runs in production.",
                "Sí, si la caja se reconstruye sola. Se describen los datos nuevos en un archivo y \
                 los controles, los modelos que compiten, el tablero y las exportaciones toman esa \
                 forma. Cinco proyectos empezaron así; uno corre en producción.",
            ),
            Question::Material => t(
                "Comparing a measured diffraction pattern against a database of known structures \
                 turns \"probably\" into a ranked shortlist — the difference between a guess and a \
                 candidate worth testing.",
                "Comparar un patrón de difracción medido contra una base de estructuras conocidas \
                 convierte un \"probablemente\" en una lista corta y ordenada: la diferencia entre \
                 una corazonada y un candidato que vale la pena poner a prueba.",
            ),
            Question::FromPaper => t(
                "Three times over, in languages the original authors never used. The algorithm is \
                 never the hard part; the hard part is the step the paper leaves out because the \
                 authors thought it obvious.",
                "Tres veces, en lenguajes que los autores originales nunca usaron. El algoritmo \
                 nunca es la parte difícil; la parte difícil es el paso que el paper omite porque \
                 a los autores les parecía obvio.",
            ),
            Question::Textbook => t(
                "With retrieval, yes — and instrumenting it mattered more than the wording of the \
                 prompts. Every answer traces back to the passages that produced it.",
                "Con recuperación, sí, e instrumentarlo importó más que la redacción de los \
                 prompts. Cada respuesta se puede rastrear hasta los pasajes que la produjeron.",
            ),
            Question::MachineDraws => t(
                "No brief, no client, no score to beat — the only work here that exists purely to \
                 be looked at. Random walkers piling up over a canvas, and sound turned into a \
                 landscape you can mix.",
                "Sin encargo, sin cliente, sin puntaje que superar: el único trabajo acá que \
                 existe solo para ser mirado. Caminantes aleatorios acumulándose sobre un lienzo, \
                 y sonido convertido en un paisaje que se puede mezclar.",
            ),
        }
        .get(spanish)
    }
}

/// A short factual marker on a row. Facts only — never an adjective.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tick {
    Live,
    Private,
    MostlyPrivate,
    Unattended,
    Teaching,
    Since2021,
}

impl Tick {
    pub fn label(&self, spanish: bool) -> &'static str {
        match self {
            Tick::Live => t("live", "en vivo"),
            Tick::Private => t("private", "privado"),
            Tick::MostlyPrivate => t("mostly private", "casi todo privado"),
            Tick::Unattended => t("runs unattended", "corre sin supervisión"),
            Tick::Teaching => t("teaching", "docencia"),
            Tick::Since2021 => t("2021", "2021"),
        }
        .get(spanish)
    }

    /// The one marker that earns the accent colour.
    pub fn is_live(&self) -> bool {
        matches!(self, Tick::Live)
    }
}

#[derive(Clone, Copy)]
pub struct Link {
    pub label: Text,
    pub url: &'static str,
}

/// The live thing itself.
const fn open(url: &'static str) -> Link {
    Link {
        label: t("Open", "Abrir"),
        url,
    }
}

/// The source.
const fn github(url: &'static str) -> Link {
    Link {
        label: t("GitHub", "GitHub"),
        url,
    }
}

/// Anything else — a paper, a package, a write-up. Label it in both languages.
/// Unused until a row needs it; kept so adding one stays a one-line edit.
#[allow(dead_code)]
const fn link(en: &'static str, es: &'static str, url: &'static str) -> Link {
    Link {
        label: t(en, es),
        url,
    }
}

#[derive(Clone, Copy)]
pub struct Project {
    pub name: Text,
    /// One sentence. If it needs two, the project needs a case study instead.
    pub line: Text,
    pub tech: &'static [&'static str],
    pub ticks: &'static [Tick],
    /// Repositories folded into this row, shown as a quiet "+n".
    pub tail: &'static [&'static str],
    /// Every way in: the live thing, the source, a paper. Empty when there is
    /// nowhere public to send anyone.
    pub links: &'static [Link],
    pub layer: Layer,
    pub question: Option<Question>,
}

pub const PROJECTS: &[Project] = &[
    // ── what you see ────────────────────────────────────────────────────────
    Project {
        name: t("Infinite Playlist", "Infinite Playlist"),
        line: t(
            "Seed it with any song and it never runs out — the spotify-next-track model, running \
             in your browser.",
            "Se le da una canción cualquiera y no se agota nunca: el modelo de spotify-next-track \
             corriendo en el navegador.",
        ),
        tech: &["ONNX Runtime Web", "Rust/WASM", "Cloudflare Workers"],
        ticks: &[Tick::Live],
        tail: &[],
        links: &[open("https://infinite-playlist.eduardo-gonik.workers.dev")],
        layer: Layer::See,
        question: Some(Question::HearNext),
    },
    Project {
        name: t("Pathfinder", "Pathfinder"),
        line: t(
            "The route between two songs, drawn in 3D and scored hop by hop by the taste-fit model.",
            "La ruta entre dos canciones, dibujada en 3D y puntuada salto por salto por el modelo \
             de afinidad.",
        ),
        tech: &["Rust/WASM", "React", "Three.js", "ONNX"],
        ticks: &[Tick::Live],
        tail: &[],
        links: &[open("https://pathfinder.eduardo-gonik.workers.dev/")],
        layer: Layer::See,
        question: Some(Question::HearNext),
    },
    Project {
        name: t("demoLogits", "demoLogits"),
        line: t(
            "Watch a language model choose its next word, one candidate at a time.",
            "Ver a un modelo de lenguaje elegir su próxima palabra, candidata por candidata.",
        ),
        tech: &["JavaScript", "Cloudflare Workers"],
        ticks: &[Tick::Live],
        tail: &[],
        links: &[open("https://demo-logits.eduardo-gonik.workers.dev")],
        layer: Layer::See,
        question: Some(Question::PickWord),
    },
    Project {
        name: t("songViz", "songViz"),
        line: t(
            "Sound turned into a landscape, two tracks at a time, with a crossfader.",
            "Sonido convertido en paisaje, dos canciones a la vez, con un crossfader.",
        ),
        tech: &["JavaScript", "Web Audio", "Three.js"],
        ticks: &[],
        tail: &[],
        links: &[github("https://github.com/egonik-unlp/songVizClaude")],
        layer: Layer::See,
        question: Some(Question::MachineDraws),
    },
    Project {
        name: t("flagen", "flagen"),
        line: t(
            "Pictures built out of repeated random decisions.",
            "Imágenes construidas a partir de decisiones aleatorias repetidas.",
        ),
        tech: &["OCaml", "Owl", "Monte Carlo"],
        ticks: &[],
        tail: &["rust_screen_background"],
        links: &[github("https://github.com/egonik-unlp/flagen")],
        layer: Layer::See,
        question: Some(Question::MachineDraws),
    },
    Project {
        name: t("demoCodex", "demoCodex"),
        line: t(
            "A calorimetry correction shown being made instead of asserted.",
            "Una corrección calorimétrica que se muestra mientras se hace, en lugar de afirmarse.",
        ),
        tech: &["TypeScript"],
        ticks: &[Tick::Teaching],
        tail: &["fit-anim"],
        links: &[github("https://github.com/egonik-unlp/demoCodex")],
        layer: Layer::See,
        question: None,
    },
    Project {
        name: t("This site", "Este sitio"),
        line: t(
            "A portfolio in Rust, on the fifth attempt — and no external requests.",
            "Un portfolio en Rust, al quinto intento, y sin pedidos a servidores externos.",
        ),
        tech: &["Rust", "Leptos", "WebAssembly", "Axum"],
        ticks: &[],
        tail: &["website", "massive",  "proyecto_eframe"],
        links: &[github("https://github.com/egonik-unlp/egonik-unlp.github.io")],
        layer: Layer::See,
        question: None,
    },
    Project {
        name: t("My future site", "Mi próximo sitio"),
        line: t(
            "My fullstack rust website with leptos ssr",
            "Mi sitio en rust fullstack usando leptos ssr",
        ),
        tech: &["Rust", "Leptos", "WebAssembly", "Actix","Tokio" ],
        ticks: &[],
        tail: &[],
        links: &[
            open("https://site-production-20aa.up.railway.app/"),
            github("https://github.com/egonik-unlp/egonik-site"),
        ],
        layer: Layer::See,
        question: None,
    },
    // ── what you use ────────────────────────────────────────────────────────
    Project {
        name: t("Lensing", "Lensing"),
        line: t(
            "A prediction toolkit that rebuilds itself around whatever data you point it at.",
            "Una caja de herramientas de predicción que se reconstruye alrededor de los datos que \
             se le indiquen.",
        ),
        tech: &["Rust", "burn", "Python", "Julia", "Qdrant", "ONNX", "Zig"],
        ticks: &[],
        tail: &[
            "price-guesser-models",
            "listing-anomaly-detection",
            "turbine-alarm-forecast",
            "lensing-workspace",
        ],
        links: &[github("https://github.com/egonik-unlp/lensing")],
        layer: Layer::Use,
        question: Some(Question::Toolkit),
    },
    Project {
        name: t("craig", "craig"),
        line: t(
            "A chemistry textbook you can ask questions, with every answer traceable.",
            "Un libro de fisicoquímica al que se le pueden hacer preguntas, con cada respuesta \
             rastreable.",
        ),
        tech: &["TypeScript", "LangChain", "Qdrant", "Langfuse"],
        ticks: &[],
        tail: &[],
        links: &[github("https://github.com/egonik-unlp/craig")],
        layer: Layer::Use,
        question: Some(Question::Textbook),
    },
    Project {
        name: t("convert-ffi", "convert-ffi"),
        line: t(
            "The browser reads your audio tags, then Rust hands them to a Zig Spotify client \
             through FFI.",
            "El navegador lee las etiquetas de tus archivos y Rust se las pasa a un cliente de \
             Spotify en Zig por FFI.",
        ),
        tech: &["Rust", "Leptos", "Zig", "FFI"],
        ticks: &[Tick::Live],
        tail: &[],
        links: &[open("https://convert-ffi-latest.onrender.com/")],
        layer: Layer::Use,
        question: Some(Question::Collection),
    },
    Project {
        name: t("glotaran converters", "conversores para Glotaran"),
        line: t(
            "Fluorescence and flash-photolysis files turned into datasets the standard tools can \
             open.",
            "Archivos de fluorescencia y fotólisis de destello convertidos en datasets que abren \
             las herramientas estándar.",
        ),
        tech: &["Rust"],
        ticks: &[],
        tail: &[
            "glotaran_converter_cli",
            "glotaran_converter",
            "glotaran_gui",
            "glotaran_preprocessing",
            "QuenchingLFP",
            "deconv_fit",
            "flim",
            "flim_tsv",
        ],
        links: &[github("https://github.com/egonik-unlp/glotaran_converter_lib")],
        layer: Layer::Use,
        question: Some(Question::Instrument),
    },
    Project {
        name: t("Work for other people", "Trabajo para otras personas"),
        line: t(
            "A pathology database, a landing page, a research group's site, analytics tooling, a \
             hiring challenge.",
            "Una base de datos de patologías, una landing, el sitio de un grupo de investigación, \
             herramientas de analytics, un challenge de selección.",
        ),
        tech: &["Go", "TypeScript", "Rust", "Python"],
        ticks: &[Tick::MostlyPrivate],
        tail: &[
            "patologias",
            "site-gustavo-gonik",
            "nanofot_sitio",
            "acomodar-db",
            "analytics",
            "analytics_v2",
            "data-generation",
            "lanacion-challenge",
            "tax_parser",
        ],
        links: &[],
        layer: Layer::Use,
        question: None,
    },
    // ── what it works out ───────────────────────────────────────────────────
    Project {
        name: t("spotify-next-track", "spotify-next-track"),
        line: t(
            "Seven rounds of experiments on 7,150 listening sessions, failures kept on the record.",
            "Siete rondas de experimentos sobre 7.150 sesiones de escucha, con los fracasos \
             anotados.",
        ),
        tech: &["Rust", "burn", "PyTorch", "Julia", "Qdrant"],
        ticks: &[],
        tail: &[],
        links: &[github("https://github.com/egonik-unlp/spotify-next-track")],
        layer: Layer::Works,
        question: Some(Question::HearNext),
    },
    Project {
        name: t("spotify-predict-engagement", "spotify-predict-engagement"),
        line: t(
            "Nine rounds proving that on catalog facts alone, plain decision trees win.",
            "Nueve rondas que muestran que, con solo datos de catálogo, ganan los árboles de \
             decisión simples.",
        ),
        tech: &["Python", "xgboost", "CatBoost", "burn"],
        ticks: &[Tick::Private],
        tail: &[],
        links: &[],
        layer: Layer::Works,
        question: Some(Question::HearNext),
    },
    Project {
        name: t("xrd_match", "xrd_match"),
        line: t(
            "A measured diffraction pattern, matched against a database of known structures.",
            "Un patrón de difracción medido, comparado contra una base de estructuras conocidas.",
        ),
        tech: &["Python", "Jupyter"],
        ticks: &[],
        tail: &["mof_xrd", "docking_cecilia", "cecilia", "labo"],
        links: &[github("https://github.com/egonik-unlp/xrd_match")],
        layer: Layer::Works,
        question: Some(Question::Material),
    },
    Project {
        name: t("MOFSocialNet.jl", "MOFSocialNet.jl"),
        line: t(
            "Metal-organic frameworks read as a social network — a published method reproduced in \
             Julia.",
            "Redes metal-orgánicas leídas como una red social: un método publicado reproducido en \
             Julia.",
        ),
        tech: &["Julia", "Jupyter"],
        ticks: &[],
        tail: &["bilinear", "PHcalc.jl"],
        links: &[github("https://github.com/egonik-unlp/MOFSocialNet.jl")],
        layer: Layer::Works,
        question: Some(Question::FromPaper),
    },
    Project {
        name: t("Molecular dynamics, twice", "Dinámica molecular, dos veces"),
        line: t(
            "The same gas of bouncing particles, written once in Julia and again in Rust.",
            "El mismo gas de partículas que rebotan, escrito una vez en Julia y otra en Rust.",
        ),
        tech: &["Julia", "Rust"],
        ticks: &[],
        tail: &["gases", "rust_MD", "fluid-dynamics-demo", "montecarlo_pi"],
        links: &[],
        layer: Layer::Works,
        question: None,
    },
    Project {
        name: t("cinetica-Julia", "cinetica-Julia"),
        line: t(
            "Chemical kinetics worked through in code rather than plotted as a finished answer.",
            "Cinética química resuelta en código en lugar de graficada como respuesta terminada.",
        ),
        tech: &["Julia"],
        ticks: &[Tick::Teaching],
        tail: &["catedra", "catedra2", "schaposnik"],
        links: &[github("https://github.com/egonik-unlp/cinetica-Julia")],
        layer: Layer::Works,
        question: None,
    },
    // ── what it knows ───────────────────────────────────────────────────────
    Project {
        name: t("GoodVibes", "GoodVibes"),
        line: t(
            "Label photographs read, ingredients matched against a database, reported by category.",
            "Fotos de etiquetas leídas, ingredientes comparados contra una base y reportados por \
             categoría.",
        ),
        tech: &["Python", "Cython", "Azure OCR"],
        ticks: &[Tick::Private, Tick::Since2021],
        tail: &[
            "label_recognition",
            "goodvibes_master",
            "goodvibes_datasets",
            "goodvibes_modular",
            "goodvibes_main",
            "goodvibes_sync",
            "goodvibes_labels",
            "goodvibes_aditivos",
            "goodvibes_output_edu",
            "api_marcas",
            "barcodes",
            "gv_xml_ui",
            "image_poster",
            "dashboard_ocr",
        ],
        links: &[],
        layer: Layer::Knows,
        question: Some(Question::Shelf),
    },
    Project {
        name: t("lvv", "lvv"),
        line: t(
            "Any dataset turned into something searchable by meaning, with caching and a queue.",
            "Cualquier dataset convertido en algo buscable por significado, con caché y una cola \
             de trabajos.",
        ),
        tech: &["Rust", "Qdrant", "OpenAI", "Ollama", "tokio"],
        ticks: &[],
        tail: &["rllm", "rllmz"],
        links: &[github("https://github.com/egonik-unlp/lvv")],
        layer: Layer::Knows,
        question: Some(Question::Toolkit),
    },
    Project {
        name: t("Agilent spectrum parsers", "Parsers de espectros Agilent"),
        line: t(
            "Infrared spectra read out of the instrument's own format, missing axis rebuilt.",
            "Espectros infrarrojos leídos del formato propio del instrumento, con el eje faltante \
             reconstruido.",
        ),
        tech: &["Rust", "Python"],
        ticks: &[],
        tail: &[
            "asp_lib",
            "asp_gui",
            "agilentaspparser",
            "agilent_ir",
            "_eem_converter",
            "plotter_eem",
        ],
        links: &[github("https://github.com/egonik-unlp/agilent_asp_parser")],
        layer: Layer::Knows,
        question: Some(Question::Instrument),
    },
    Project {
        name: t("detect-open-realstate-apis", "detect-open-realstate-apis"),
        line: t(
            "Scans property sites for undocumented data feeds, then grades each one for \
             confidence.",
            "Escanea sitios inmobiliarios buscando APIs no documentadas y califica cada una por \
             confianza.",
        ),
        tech: &["Rust", "tokio", "LangChain.js"],
        ticks: &[],
        tail: &["scrapping-conicet", "scrape_free_proxy", "dolar_historico"],
        links: &[github("https://github.com/egonik-unlp/detect-open-realstate-apis")],
        layer: Layer::Knows,
        question: None,
    },
    Project {
        name: t(
            "Listening history, four passes",
            "Historial de escucha, cuatro intentos",
        ),
        line: t(
            "Four attempts at my own streaming history — the last became a 12,000-track dataset.",
            "Cuatro intentos con mi propio historial de escucha; el último se volvió un dataset de \
             12.000 canciones.",
        ),
        tech: &["Jupyter", "Rust", "Go"],
        ticks: &[],
        tail: &[
            "spotify-data",
            "SpotifyData",
            "my-spotify-data",
            "go-albums",
            "google_fit",
        ],
        links: &[],
        layer: Layer::Knows,
        question: Some(Question::HearNext),
    },
    // ── what it runs on ─────────────────────────────────────────────────────
    Project {
        name: t("convert-invert", "convert-invert"),
        line: t(
            "A worker pool that finds each track of a playlist as a real file and judges the \
             candidates.",
            "Un pool de workers que busca cada canción de una playlist como archivo real y juzga \
             las candidatas.",
        ),
        tech: &["Rust", "Actix", "Postgres", "Redis", "OpenTelemetry"],
        ticks: &[Tick::Unattended],
        tail: &[
            "convert-invert-site",
            "convert-invert-frontend",
            "convert-host-downloads",
            "convert-site",
            "convert-ui",
            "spotify-to-youtube",
        ],
        links: &[github("https://github.com/egonik-unlp/convert-invert")],
        layer: Layer::Runs,
        question: Some(Question::Collection),
    },
    Project {
        name: t("convert-songs", "convert-songs"),
        line: t(
            "The trip in reverse, in Zig, with the login handshake written by hand.",
            "El viaje inverso, en Zig, con el handshake de login escrito a mano.",
        ),
        tech: &["Zig", "OAuth2", "ID3"],
        ticks: &[],
        tail: &[],
        links: &[github("https://github.com/egonik-unlp/convert-songs")],
        layer: Layer::Runs,
        question: Some(Question::Collection),
    },
    Project {
        name: t("dump_ps", "dump_ps"),
        line: t(
            "What was this machine doing an hour ago? Process snapshots, stored.",
            "¿Qué estaba haciendo esta máquina hace una hora? Instantáneas de procesos, guardadas.",
        ),
        tech: &["Zig", "SQLite"],
        ticks: &[],
        tail: &[],
        links: &[github("https://github.com/egonik-unlp/dump_ps")],
        layer: Layer::Runs,
        question: None,
    },
    Project {
        name: t(
            "Small servers, eleven of them",
            "Servidores chicos, once de ellos",
        ),
        line: t(
            "Each one drops a layer the last turned out not to need. Two still run something.",
            "Cada uno saca una capa que el anterior resultó no necesitar. Dos todavía corren algo.",
        ),
        tech: &["Rust", "Go", "Zig"],
        ticks: &[],
        tail: &[
            "toy-server",
            "tiny-server",
            "return-list-connects",
            "server",
            "background_task",
            "remote_listener",
            "server-shell",
            "vic-zig",
            "zig-rust",
            "envfiles",
            "import-lib",
        ],
        links: &[github("https://github.com/egonik-unlp/toy-server")],
        layer: Layer::Runs,
        question: None,
    },
];

/// Projects in a layer, in declaration order.
pub fn by_layer(layer: Layer) -> Vec<&'static Project> {
    PROJECTS.iter().filter(|p| p.layer == layer).collect()
}

/// Projects under a question, in declaration order.
pub fn by_question(question: Question) -> Vec<&'static Project> {
    PROJECTS
        .iter()
        .filter(|p| p.question == Some(question))
        .collect()
}

/// Everything that answers no question of its own — demoted, never dropped.
/// Cutting a question from [`Question::ordered`] moves its projects here.
pub fn rest() -> Vec<&'static Project> {
    let listed = Question::ordered();
    PROJECTS
        .iter()
        .filter(|p| !p.question.is_some_and(|q| listed.contains(&q)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Both readings must cover the index: no empty group, nothing dropped.
    #[test]
    fn every_layer_and_question_has_projects() {
        for layer in Layer::ordered() {
            assert!(
                !by_layer(layer).is_empty(),
                "layer \"{}\" has no projects",
                layer.label(false)
            );
        }
        for question in Question::ordered() {
            assert!(
                !by_question(question).is_empty(),
                "question \"{}\" has no projects",
                question.text(false)
            );
        }
    }

    /// Every project appears exactly once in the layer view, and exactly once in
    /// the question view — under a question or in the closing group.
    #[test]
    fn question_view_accounts_for_every_project() {
        let in_layers: usize = Layer::ordered().iter().map(|l| by_layer(*l).len()).sum();
        let under_questions: usize = Question::ordered()
            .iter()
            .map(|q| by_question(*q).len())
            .sum();
        assert_eq!(in_layers, PROJECTS.len());
        assert_eq!(under_questions + rest().len(), PROJECTS.len());
    }

    /// A row's sentence stays a sentence, in both languages, and a row that
    /// promises "live" has somewhere to go.
    #[test]
    fn rows_stay_one_line_in_both_languages() {
        for p in PROJECTS {
            for (lang, line) in [("en", p.line.en), ("es", p.line.es)] {
                assert!(
                    line.chars().count() <= 145,
                    "{} needs a shorter {lang} line ({} chars)",
                    p.name.en,
                    line.chars().count()
                );
            }
            assert!(!p.name.es.is_empty(), "{} has no Spanish name", p.name.en);
            if p.ticks.iter().any(Tick::is_live) {
                assert!(
                    !p.links.is_empty(),
                    "{} is marked live with no link",
                    p.name.en
                );
            }
        }
    }

    /// A row offers a few ways in, not a link farm: three at most, each going
    /// somewhere different, each labelled distinctly.
    #[test]
    fn links_stay_a_short_list() {
        for p in PROJECTS {
            assert!(
                p.links.len() <= 3,
                "{} has {} links — fold some into the tail",
                p.name.en,
                p.links.len()
            );
            for (i, l) in p.links.iter().enumerate() {
                for other in &p.links[i + 1..] {
                    assert_ne!(l.url, other.url, "{} repeats a url", p.name.en);
                    assert_ne!(
                        l.label.en, other.label.en,
                        "{} has two links labelled \"{}\"",
                        p.name.en, l.label.en
                    );
                }
            }
        }
    }

    /// Nothing is listed twice: a folded repo is never also a headline row.
    #[test]
    fn tails_do_not_repeat_a_headline() {
        let names: Vec<&str> = PROJECTS.iter().map(|p| p.name.en).collect();
        for p in PROJECTS {
            for repo in p.tail {
                assert!(
                    !names.contains(repo),
                    "{repo} is both a row and folded into {}",
                    p.name.en
                );
            }
        }
    }
}
