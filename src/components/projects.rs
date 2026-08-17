use leptos::prelude::*;

use crate::data::projects::{by_layer, by_question, rest, Layer, Project, Question, PROJECTS};

/// Which reading of the index is on screen. Layers are the default because they
/// need no reading; questions are there for the visitor who wants the point.
#[derive(Clone, Copy, PartialEq, Eq)]
enum View {
    Layers,
    Questions,
}

#[component]
pub fn Projects(spanish: bool) -> impl IntoView {
    let (view, set_view) = signal(View::Layers);

    let body = move || match view.get() {
        View::Layers => Layer::ordered()
            .into_iter()
            .filter_map(|l| {
                let items = by_layer(l);
                (!items.is_empty()).then(|| group(l.label(spanish), items, spanish))
            })
            .collect_view()
            .into_any(),
        View::Questions => {
            let asked = Question::ordered()
                .into_iter()
                .filter_map(|q| {
                    let items = by_question(q);
                    (!items.is_empty()).then(|| question_group(q, items, spanish))
                })
                .collect_view();
            let leftovers = rest();
            let closing = (!leftovers.is_empty())
                .then(|| group(Question::rest_label(spanish), leftovers, spanish));
            view! { {asked} {closing} }.into_any()
        }
    };

    view! {
        <section id="work" class="section">
            <div class="container">
                <div class="section-head" data-reveal="">
                    <h2 class="section-title">
                        {if spanish {
                            "Desde la pantalla hasta "
                        } else {
                            "From the screen down to "
                        }}
                        <span class="lit">
                            {if spanish { "la máquina" } else { "the machine" }}
                        </span>
                    </h2>
                    <div class="section-aside">
                        <p class="section-lede">
                            {move || match (view.get(), spanish) {
                                (View::Layers, false) => {
                                    format!(
                                        "{} projects, ordered from what you look at to what keeps \
                                         it running.",
                                        PROJECTS.len(),
                                    )
                                }
                                (View::Layers, true) => {
                                    format!(
                                        "{} proyectos, ordenados desde lo que se mira hasta lo que \
                                         lo mantiene funcionando.",
                                        PROJECTS.len(),
                                    )
                                }
                                (View::Questions, false) => {
                                    format!(
                                        "The same {} projects, grouped by what each one set out to \
                                         answer.",
                                        PROJECTS.len(),
                                    )
                                }
                                (View::Questions, true) => {
                                    format!(
                                        "Los mismos {} proyectos, agrupados según lo que cada uno \
                                         buscaba responder.",
                                        PROJECTS.len(),
                                    )
                                }
                            }}
                        </p>
                        <div
                            class="views"
                            role="group"
                            aria-label=if spanish {
                                "Elegí cómo leer el trabajo"
                            } else {
                                "Choose how to read the work"
                            }
                        >
                            <button
                                type="button"
                                aria-pressed=move || (view.get() == View::Layers).to_string()
                                on:click=move |_| set_view.set(View::Layers)
                            >
                                {if spanish { "por capa" } else { "by layer" }}
                            </button>
                            <button
                                type="button"
                                aria-pressed=move || (view.get() == View::Questions).to_string()
                                on:click=move |_| set_view.set(View::Questions)
                            >
                                {if spanish { "por pregunta" } else { "by question" }}
                            </button>
                        </div>
                    </div>
                </div>
                {body}
            </div>
        </section>
    }
}

/// A labelled run of rows. The label carries the whole explanation.
fn group(label: &'static str, items: Vec<&'static Project>, spanish: bool) -> impl IntoView {
    let rows = items
        .into_iter()
        .enumerate()
        .map(|(i, p)| work_row(p, i, spanish))
        .collect_view();

    view! {
        <div class="work-group">
            <p class="group-label">{label}</p>
            {rows}
        </div>
    }
}
fn question_group(q: Question, items: Vec<&'static Project>, spanish: bool) -> impl IntoView {
    let rows = items
        .into_iter()
        .enumerate()
        .map(|(i, p)| work_row(p, i, spanish))
        .collect_view();

    view! {
        <div class="work-group q-group">
            <h3 class="q-title">{q.text(spanish)}</h3>
            <p class="q-answer">{q.answer(spanish)}</p>
            {rows}
        </div>
    }
}

/// One project, one line: name and markers, the sentence, the tools.
fn work_row(p: &'static Project, i: usize, spanish: bool) -> impl IntoView {
    let number = format!("{:02}", i + 1);
    let delay = format!("{}", i * 45);

    let ticks = p
        .ticks
        .iter()
        .map(|tick| {
            view! {
                <span class="badge" class:is-live=tick.is_live()>
                    {tick.label(spanish)}
                </span>
            }
        })
        .collect_view();

    let tail = (!p.tail.is_empty()).then(|| {
        view! {
            <span class="more" title=p.tail.join(" · ")>
                {format!("+{}", p.tail.len())}
            </span>
        }
    });

    let links = (!p.links.is_empty()).then(|| {
        let items = p
            .links
            .iter()
            .map(|l| {
                view! {
                    <a class="work-link" href=l.url target="_blank" rel="noopener noreferrer">
                        {l.label.get(spanish)}
                        <span class="arr" aria-hidden="true">" ↗"</span>
                    </a>
                }
            })
            .collect_view();
        view! { <nav class="work-links" aria-label=p.name.get(spanish)>{items}</nav> }
    });

    let tech = p
        .tech
        .iter()
        .map(|item| view! { <li>{*item}</li> })
        .collect_view();

    view! {
        <article class="work-row" data-reveal="" data-reveal-delay=delay>
            <span class="work-index" aria-hidden="true">{number}</span>
            <div class="work-head">
                <h4 class="work-name">{p.name.get(spanish)} {tail}</h4>
                {ticks}
                {links}
            </div>
            <div class="work-body">
                <p class="work-desc">{p.line.get(spanish)}</p>
                <ul class="tech" aria-label=if spanish { "Tecnologías" } else { "Tech stack" }>
                    {tech}
                </ul>
            </div>
        </article>
    }
}
