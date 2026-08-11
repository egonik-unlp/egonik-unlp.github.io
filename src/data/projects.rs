//! Portfolio content. Editing the site is mostly editing this file.
//!
//! One index, two readings. Every project declares the [`Layer`] it sits in —
//! how far from the screen it lives — and, if it earns one, the [`Question`] it
//! set out to answer. A project without a question is not hidden: it falls into
//! the closing group at the end of the question view.

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
    pub fn label(&self) -> &'static str {
        match self {
            Layer::See => "what you see",
            Layer::Use => "what you use",
            Layer::Works => "what it works out",
            Layer::Knows => "what it knows",
            Layer::Runs => "what it runs on",
        }
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
    /// Label for the group that collects projects with no question of their own.
    pub const REST: &'static str = "the rest of the shelf";

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

    pub fn text(&self) -> &'static str {
        match self {
            Question::HearNext => "Can a computer guess what you'll want to hear next?",
            Question::Shelf => "What is actually in the products on the shelf?",
            Question::Instrument => {
                "Can a lab instrument's output become something anyone can read?"
            }
            Question::Collection => {
                "What does it take to keep a music collection that outlives the service?"
            }
            Question::PickWord => "What is a language model doing when it picks a word?",
            Question::Toolkit => "Can one toolkit be pointed at any prediction problem?",
            Question::Material => "What material am I actually holding?",
            Question::FromPaper => {
                "Can a method from a scientific paper become software other people run?"
            }
            Question::Textbook => "Can a computer answer questions about a real textbook?",
            Question::MachineDraws => "What comes out if you let the machine draw?",
        }
    }

    pub fn answer(&self) -> &'static str {
        match self {
            Question::HearNext => {
                "Partly — and only by combining three unlike methods. A neural model on its own \
                 ties a simple tally of what usually follows what; add a \"sounds similar\" pass \
                 and they land the real next song in the top ten about 21% of the time, 61% better \
                 than the tally alone. Making the network cleverer never once helped."
            }
            Question::Shelf => {
                "Answerable, but only by building the whole chain: photograph the label, read the \
                 text off it, match the printed ingredients against a curated database, report by \
                 category. The first project big enough to need a dozen repositories, back in 2021."
            }
            Question::Instrument => {
                "That is what most of five years of lab code was for. Every machine writes its own \
                 private format and the knowledge of how to read it lives in one person's head; \
                 these move it into software, so a measurement can be re-analysed years later by \
                 someone else."
            }
            Question::Collection => {
                "More than a script. A pool of workers, a judge deciding which of several candidate \
                 files is really the track you asked for, retries, and tracing to see where things \
                 stall. It has run unattended on a home server ever since."
            }
            Question::PickWord => {
                "Choosing between a handful of candidates, over and over. Obvious the moment you \
                 animate it, invisible written down — which is the whole argument for building the \
                 demo."
            }
            Question::Toolkit => {
                "Yes, if the toolkit rebuilds itself. Describe the new data in one file and the \
                 controls, the competing models, the dashboard and the exports all take its shape. \
                 Five projects started this way; one runs in production."
            }
            Question::Material => {
                "Comparing a measured diffraction pattern against a database of known structures \
                 turns \"probably\" into a ranked shortlist — the difference between a guess and a \
                 candidate worth testing."
            }
            Question::FromPaper => {
                "Three times over, in languages the original authors never used. The algorithm is \
                 never the hard part; the hard part is the step the paper leaves out because the \
                 authors thought it obvious."
            }
            Question::Textbook => {
                "With retrieval, yes — and instrumenting it mattered more than the wording of the \
                 prompts. Every answer traces back to the passages that produced it."
            }
            Question::MachineDraws => {
                "No brief, no client, no score to beat — the only work here that exists purely to \
                 be looked at. Random walkers piling up over a canvas, and sound turned into a \
                 landscape you can mix."
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct Link {
    pub label: &'static str,
    pub url: &'static str,
}

#[derive(Clone, Copy)]
pub struct Project {
    pub name: &'static str,
    /// One sentence. If it needs two, the project needs a case study instead.
    pub line: &'static str,
    /// Tools, as data — this is why there is no separate stack list in the index.
    pub tech: &'static str,
    /// Short factual markers: "live", "private", "runs unattended".
    pub ticks: &'static [&'static str],
    /// Repositories folded into this row, shown as a quiet "+n".
    pub tail: &'static [&'static str],
    pub link: Option<Link>,
    pub layer: Layer,
    pub question: Option<Question>,
}

pub const PROJECTS: &[Project] = &[
    // ── what you see ────────────────────────────────────────────────────────
    Project {
        name: "Infinite Playlist",
        line: "Seed it with any song and it never runs out — the spotify-next-track model, \
               running in your browser.",
        tech: "ONNX Web · Rust/WASM",
        ticks: &["live"],
        tail: &[],
        link: Some(Link {
            label: "Open",
            url: "https://infinite-playlist.eduardo-gonik.workers.dev",
        }),
        layer: Layer::See,
        question: Some(Question::HearNext),
    },
    Project {
        name: "Pathfinder",
        line: "The route between two songs, drawn in 3D and scored hop by hop by the taste-fit \
               model.",
        tech: "Rust/WASM · Three.js",
        ticks: &["live"],
        tail: &[],
        link: Some(Link {
            label: "Open",
            url: "https://pathfinder.eduardo-gonik.workers.dev/",
        }),
        layer: Layer::See,
        question: Some(Question::HearNext),
    },
    Project {
        name: "demoLogits",
        line: "Watch a language model choose its next word, one candidate at a time.",
        tech: "JS · Cloudflare Workers",
        ticks: &["live"],
        tail: &[],
        link: Some(Link {
            label: "Open",
            url: "https://demo-logits.eduardo-gonik.workers.dev",
        }),
        layer: Layer::See,
        question: Some(Question::PickWord),
    },
    Project {
        name: "songViz",
        line: "Sound turned into a landscape, two tracks at a time, with a crossfader.",
        tech: "Web Audio · Three.js",
        ticks: &[],
        tail: &[],
        link: Some(Link {
            label: "GitHub",
            url: "https://github.com/egonik-unlp/songVizClaude",
        }),
        layer: Layer::See,
        question: Some(Question::MachineDraws),
    },
    Project {
        name: "flagen",
        line: "Pictures built out of repeated random decisions.",
        tech: "OCaml · Owl",
        ticks: &[],
        tail: &["rust_screen_background"],
        link: Some(Link {
            label: "GitHub",
            url: "https://github.com/egonik-unlp/flagen",
        }),
        layer: Layer::See,
        question: Some(Question::MachineDraws),
    },
    Project {
        name: "demoCodex",
        line: "A calorimetry correction shown being made instead of asserted.",
        tech: "TypeScript",
        ticks: &["teaching"],
        tail: &["fit-anim"],
        link: Some(Link {
            label: "GitHub",
            url: "https://github.com/egonik-unlp/demoCodex",
        }),
        layer: Layer::See,
        question: None,
    },
    Project {
        name: "This site",
        line: "A portfolio in Rust, on the fifth attempt — and no external requests.",
        tech: "Leptos · WASM · Axum",
        ticks: &[],
        tail: &["website", "massive", "egonik-site", "proyecto_eframe"],
        link: Some(Link {
            label: "GitHub",
            url: "https://github.com/egonik-unlp/egonik-unlp.github.io",
        }),
        layer: Layer::See,
        question: None,
    },
    // ── what you use ────────────────────────────────────────────────────────
    Project {
        name: "Lensing",
        line: "A prediction toolkit that rebuilds itself around whatever data you point it at.",
        tech: "Rust · Python · Julia · Zig",
        ticks: &[],
        tail: &[
            "price-guesser-models",
            "listing-anomaly-detection",
            "turbine-alarm-forecast",
            "lensing-workspace",
        ],
        link: Some(Link {
            label: "GitHub",
            url: "https://github.com/egonik-unlp/lensing",
        }),
        layer: Layer::Use,
        question: Some(Question::Toolkit),
    },
    Project {
        name: "craig",
        line: "A chemistry textbook you can ask questions, with every answer traceable.",
        tech: "TypeScript · Qdrant",
        ticks: &[],
        tail: &[],
        link: Some(Link {
            label: "GitHub",
            url: "https://github.com/egonik-unlp/craig",
        }),
        layer: Layer::Use,
        question: Some(Question::Textbook),
    },
    Project {
        name: "glotaran converters",
        line: "Fluorescence and flash-photolysis files turned into datasets the standard tools \
               can open.",
        tech: "Rust",
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
        link: Some(Link {
            label: "GitHub",
            url: "https://github.com/egonik-unlp/glotaran_converter_lib",
        }),
        layer: Layer::Use,
        question: Some(Question::Instrument),
    },
    Project {
        name: "Work for other people",
        line: "A pathology database, a landing page, a group's website, analytics tooling, a \
               hiring challenge.",
        tech: "Go · TypeScript · Rust",
        ticks: &["mostly private"],
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
        link: None,
        layer: Layer::Use,
        question: None,
    },
    // ── what it works out ───────────────────────────────────────────────────
    Project {
        name: "spotify-next-track",
        line: "Seven rounds of experiments on 7,150 listening sessions, failures kept on the \
               record.",
        tech: "Rust · PyTorch · Julia",
        ticks: &[],
        tail: &[],
        link: Some(Link {
            label: "GitHub",
            url: "https://github.com/egonik-unlp/spotify-next-track",
        }),
        layer: Layer::Works,
        question: Some(Question::HearNext),
    },
    Project {
        name: "spotify-predict-engagement",
        line: "Nine rounds proving that on catalog facts alone, plain decision trees win.",
        tech: "Python · xgboost · burn",
        ticks: &["private"],
        tail: &[],
        link: None,
        layer: Layer::Works,
        question: Some(Question::HearNext),
    },
    Project {
        name: "xrd_match",
        line: "A measured diffraction pattern, matched against a database of known structures.",
        tech: "Python · Jupyter",
        ticks: &[],
        tail: &["mof_xrd", "docking_cecilia", "cecilia", "labo"],
        link: Some(Link {
            label: "GitHub",
            url: "https://github.com/egonik-unlp/xrd_match",
        }),
        layer: Layer::Works,
        question: Some(Question::Material),
    },
    Project {
        name: "MOFSocialNet.jl",
        line: "Metal-organic frameworks read as a social network — a published method reproduced \
               in Julia.",
        tech: "Julia · Jupyter",
        ticks: &[],
        tail: &["bilinear", "PHcalc.jl"],
        link: Some(Link {
            label: "GitHub",
            url: "https://github.com/egonik-unlp/MOFSocialNet.jl",
        }),
        layer: Layer::Works,
        question: Some(Question::FromPaper),
    },
    Project {
        name: "Molecular dynamics, twice",
        line: "The same gas of bouncing particles, written once in Julia and again in Rust.",
        tech: "Julia · Rust",
        ticks: &[],
        tail: &["gases", "rust_MD", "fluid-dynamics-demo", "montecarlo_pi"],
        link: None,
        layer: Layer::Works,
        question: None,
    },
    Project {
        name: "cinetica-Julia",
        line: "Chemical kinetics worked through in code rather than plotted as a finished answer.",
        tech: "Julia",
        ticks: &["teaching"],
        tail: &["catedra", "catedra2", "schaposnik"],
        link: Some(Link {
            label: "GitHub",
            url: "https://github.com/egonik-unlp/cinetica-Julia",
        }),
        layer: Layer::Works,
        question: None,
    },
    // ── what it knows ───────────────────────────────────────────────────────
    Project {
        name: "GoodVibes",
        line: "Label photographs read, ingredients matched against a database, reported by \
               category.",
        tech: "Python · Cython · OCR",
        ticks: &["private", "2021"],
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
        link: None,
        layer: Layer::Knows,
        question: Some(Question::Shelf),
    },
    Project {
        name: "lvv",
        line: "Any dataset turned into something searchable by meaning, with caching and a queue.",
        tech: "Rust · Qdrant",
        ticks: &[],
        tail: &["rllm", "rllmz"],
        link: Some(Link {
            label: "GitHub",
            url: "https://github.com/egonik-unlp/lvv",
        }),
        layer: Layer::Knows,
        question: Some(Question::Toolkit),
    },
    Project {
        name: "Agilent spectrum parsers",
        line: "Infrared spectra read out of the instrument's own format, missing axis rebuilt.",
        tech: "Rust · Python",
        ticks: &[],
        tail: &[
            "asp_lib",
            "asp_gui",
            "agilentaspparser",
            "agilent_ir",
            "_eem_converter",
            "plotter_eem",
        ],
        link: Some(Link {
            label: "GitHub",
            url: "https://github.com/egonik-unlp/agilent_asp_parser",
        }),
        layer: Layer::Knows,
        question: Some(Question::Instrument),
    },
    Project {
        name: "detect-open-realstate-apis",
        line: "Scans property sites for undocumented data feeds, then grades each one for \
               confidence.",
        tech: "Rust · LangChain.js",
        ticks: &[],
        tail: &["scrapping-conicet", "scrape_free_proxy", "dolar_historico"],
        link: Some(Link {
            label: "GitHub",
            url: "https://github.com/egonik-unlp/detect-open-realstate-apis",
        }),
        layer: Layer::Knows,
        question: None,
    },
    Project {
        name: "Listening history, four passes",
        line: "Four attempts at my own streaming history — the last became a 12,000-track dataset.",
        tech: "Jupyter · Rust · Go",
        ticks: &[],
        tail: &[
            "spotify-data",
            "SpotifyData",
            "my-spotify-data",
            "go-albums",
            "google_fit",
        ],
        link: None,
        layer: Layer::Knows,
        question: Some(Question::HearNext),
    },
    // ── what it runs on ─────────────────────────────────────────────────────
    Project {
        name: "convert-invert",
        line: "A worker pool that finds each track of a playlist as a real file and judges the \
               candidates.",
        tech: "Rust · Postgres · Redis",
        ticks: &["runs unattended"],
        tail: &[
            "convert-invert-site",
            "convert-invert-frontend",
            "convert-host-downloads",
            "convert-ffi",
            "convert-site",
            "convert-ui",
            "spotify-to-youtube",
        ],
        link: Some(Link {
            label: "GitHub",
            url: "https://github.com/egonik-unlp/convert-invert",
        }),
        layer: Layer::Runs,
        question: Some(Question::Collection),
    },
    Project {
        name: "convert-songs",
        line: "The trip in reverse, in Zig, with the login handshake written by hand.",
        tech: "Zig",
        ticks: &[],
        tail: &[],
        link: Some(Link {
            label: "GitHub",
            url: "https://github.com/egonik-unlp/convert-songs",
        }),
        layer: Layer::Runs,
        question: Some(Question::Collection),
    },
    Project {
        name: "dump_ps",
        line: "What was this machine doing an hour ago? Process snapshots, stored.",
        tech: "Zig · SQLite",
        ticks: &[],
        tail: &[],
        link: Some(Link {
            label: "GitHub",
            url: "https://github.com/egonik-unlp/dump_ps",
        }),
        layer: Layer::Runs,
        question: None,
    },
    Project {
        name: "Small servers, eleven of them",
        line: "Each one drops a layer the last turned out not to need. Two still run something.",
        tech: "Rust · Go · Zig",
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
        link: None,
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
                layer.label()
            );
        }
        for question in Question::ordered() {
            assert!(
                !by_question(question).is_empty(),
                "question \"{}\" has no projects",
                question.text()
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

    /// A row's sentence is one sentence; a link that promises "live" must exist.
    #[test]
    fn rows_stay_one_line() {
        for p in PROJECTS {
            assert!(
                p.line.len() <= 130,
                "{} needs a shorter line ({} chars)",
                p.name,
                p.line.len()
            );
            if p.ticks.contains(&"live") {
                assert!(p.link.is_some(), "{} is marked live with no link", p.name);
            }
        }
    }
}
