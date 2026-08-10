use leptos::prelude::*;

const SCHOLAR_URL: &str = "https://scholar.google.com/citations?hl=en&user=0CAay5kAAAAJ";

struct Pub {
    year: &'static str,
    title: &'static str,
    venue: &'static str,
}

// Peer-reviewed work on the photophysics of silicon-based and hybrid nanomaterials
// (INIFTA / INTECH, La Plata). Full list on Google Scholar.
const PUBS: &[Pub] = &[
    Pub {
        year: "2026",
        title: "Development of hybrid nanoparticles based on Zr(iv) and \
                perylene-3,4,9,10-tetracarboxylic acid",
        venue: "RSC Advances",
    },
    Pub {
        year: "2022",
        title: "Incorporation of N and O into the shell of silicon nanoparticles offers \
                tunable photoluminescence for imaging uses",
        venue: "ACS Applied Nano Materials",
    },
    Pub {
        year: "2022",
        title: "Optimal silicon-based nanomaterials for biological applications",
        venue: "Concepts and Design of Materials Nanoarchitectonics (RSC), book chapter",
    },
    Pub {
        year: "2021",
        title: "Environmentally induced changes of commercial carbon nanotubes in aqueous \
                suspensions",
        venue: "ACS Omega",
    },
    Pub {
        year: "2020",
        title: "Staphylococcus aureus biofilm eradication by PEG-coated silicon dots immobilized \
                in silica films and light irradiation",
        venue: "Nanotechnology",
    },
];

struct Tool {
    title: &'static str,
    body: &'static str,
    repo: Option<&'static str>,
}

const TOOLING: &[Tool] = &[
    Tool {
        title: "Spectroscopy tooling",
        body: "Converters and parsers that turn instrument output into analysable data — \
               LFP/TRES files for Glotaran, and Agilent FTIR .asp files.",
        repo: Some("https://github.com/egonik-unlp/glotaran_converter_lib"),
    },
    Tool {
        title: "MOF network analysis",
        body: "A Julia notebook that reproduces the MOFSocialNet method for comparing \
               metal-organic frameworks as a network.",
        repo: Some("https://github.com/egonik-unlp/MOFSocialNet.jl"),
    },
    Tool {
        title: "Crystallography",
        body:
            "Matching measured X-ray diffraction patterns against a CIF-based structure database.",
        repo: Some("https://github.com/egonik-unlp/xrd_match"),
    },
    Tool {
        title: "AI × chemistry",
        body: "craig, a retrieval-augmented assistant over a physical-chemistry textbook — where \
               the research and engineering threads meet.",
        repo: Some("https://github.com/egonik-unlp/craig"),
    },
];

const TOOLING_ES: &[Tool] = &[
    Tool {
        title: "Herramientas de espectroscopía",
        body: "Conversores y parsers que transforman la salida de instrumentos en datos analizables: archivos LFP/TRES para Glotaran y archivos .asp de Agilent FTIR.",
        repo: Some("https://github.com/egonik-unlp/glotaran_converter_lib"),
    },
    Tool {
        title: "Análisis de redes de MOFs",
        body: "Un notebook en Julia que reproduce el método MOFSocialNet para comparar estructuras metal-orgánicas como una red.",
        repo: Some("https://github.com/egonik-unlp/MOFSocialNet.jl"),
    },
    Tool {
        title: "Cristalografía",
        body: "Comparación de patrones de difracción de rayos X medidos contra una base de datos estructural basada en archivos CIF.",
        repo: Some("https://github.com/egonik-unlp/xrd_match"),
    },
    Tool {
        title: "IA × química",
        body: "craig, un asistente con recuperación aumentada sobre un libro de fisicoquímica: el punto de encuentro entre investigación e ingeniería.",
        repo: Some("https://github.com/egonik-unlp/craig"),
    },
];

#[component]
pub fn Research(spanish: bool) -> impl IntoView {
    let pubs = PUBS
        .iter()
        .map(|p| {
            view! {
                <li class="pub-item">
                    <span class="pub-year">{p.year}</span>
                    <span>
                        <span class="pub-title">{p.title}</span>
                        <span class="pub-venue">{p.venue}</span>
                    </span>
                </li>
            }
        })
        .collect::<Vec<_>>();

    let tooling = if spanish { TOOLING_ES } else { TOOLING }
        .iter()
        .map(|it| {
            view! {
                <li class="tool-item">
                    <h4 class="tool-title">{it.title}</h4>
                    <p>{it.body}</p>
                    {it
                        .repo
                        .map(|url| {
                            view! {
                                <a class="inline-link" href=url target="_blank" rel="noopener noreferrer">
                                    {if spanish { "Código" } else { "Code" }} <span aria-hidden="true">" ↗"</span>
                                </a>
                            }
                        })}
                </li>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <section id="research" class="section">
            <div class="container narrow">
                <div class="section-head" data-reveal="">
                    <h2 class="section-title">
                        {if spanish { "Investigación y " } else { "Research & " }}
                        <span class="lit">{if spanish { "computación científica" } else { "scientific computing" }}</span>
                    </h2>
                    <p class="section-lede">
                        {if spanish {
                            "Antes —y a la par— de la ingeniería, soy investigador doctoral en Fisicoquímica en INIFTA e INTECH, La Plata. Trabajo en la fotofísica de nanomateriales basados en silicio e híbridos para aplicaciones de imagen y antimicrobianas. Mucho de mi código nació en el laboratorio."
                        } else {
                            "Before — and alongside — the engineering, I'm a Physical-Chemistry PhD researcher at INIFTA and INTECH in La Plata, working on the photophysics of silicon-based and hybrid nanomaterials for imaging and antimicrobial uses. A lot of my code grew out of the lab."
                        }}
                    </p>
                </div>

                <div data-reveal="">
                    <p class="sub-label">{if spanish { "Publicaciones seleccionadas" } else { "Selected publications" }}</p>
                    <ul class="pub-list">{pubs}</ul>
                    <a class="inline-link" href=SCHOLAR_URL target="_blank" rel="noopener noreferrer">
                        {if spanish { "Lista completa en Google Scholar" } else { "Full list on Google Scholar" }} <span aria-hidden="true">" ↗"</span>
                    </a>
                </div>

                <div data-reveal="">
                    <p class="sub-label">{if spanish { "Herramientas de investigación" } else { "Research tooling" }}</p>
                    <ul class="tool-list">{tooling}</ul>
                </div>
            </div>
        </section>
    }
}
