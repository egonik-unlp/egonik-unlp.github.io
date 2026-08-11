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
pub fn Projects() -> impl IntoView {
    let (view, set_view) = signal(View::Layers);

    let body = move || match view.get() {
        View::Layers => Layer::ordered()
            .into_iter()
            .filter_map(|l| {
                let items = by_layer(l);
                (!items.is_empty()).then(|| group(l.label(), items))
            })
            .collect_view()
            .into_any(),
        View::Questions => {
            let asked = Question::ordered()
                .into_iter()
                .filter_map(|q| {
                    let items = by_question(q);
                    (!items.is_empty()).then(|| question_group(q, items))
                })
                .collect_view();
            let leftovers = rest();
            let closing = (!leftovers.is_empty()).then(|| group(Question::REST, leftovers));
            view! { {asked} {closing} }.into_any()
        }
    };

    view! {
        <section id="work" class="section">
            <div class="container">
                <div class="section-head" data-reveal="">
                    <h2 class="section-title">
                        "From the screen down to the " <span class="lit">"machine"</span>
                    </h2>
                    <p class="section-lede">
                        {move || match view.get() {
                            View::Layers => {
                                format!(
                                    "{} projects, ordered from what you look at to what keeps it \
                                     running.",
                                    PROJECTS.len(),
                                )
                            }
                            View::Questions => {
                                format!(
                                    "The same {} projects, grouped by what each one set out to \
                                     answer.",
                                    PROJECTS.len(),
                                )
                            }
                        }}
                    </p>
                    <div class="views">
                        <div class="seg" role="group" aria-label="Choose how to read the work">
                            <button
                                type="button"
                                aria-pressed=move || (view.get() == View::Layers).to_string()
                                on:click=move |_| set_view.set(View::Layers)
                            >
                                "by layer"
                            </button>
                            <button
                                type="button"
                                aria-pressed=move || (view.get() == View::Questions).to_string()
                                on:click=move |_| set_view.set(View::Questions)
                            >
                                "by question"
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
fn group(label: &'static str, items: Vec<&'static Project>) -> impl IntoView {
    view! {
        <div class="work-group">
            <p class="group-label">{label} <span class="after" aria-hidden="true"></span></p>
            <div class="rows">{items.into_iter().map(row).collect_view()}</div>
        </div>
    }
}

fn question_group(q: Question, items: Vec<&'static Project>) -> impl IntoView {
    view! {
        <div class="q-group">
            <h3 class="q-title">{q.text()}</h3>
            <p class="q-answer">{q.answer()}</p>
            <div class="rows">{items.into_iter().map(row).collect_view()}</div>
        </div>
    }
}

/// One project, one line: name and markers, the sentence, the tools.
fn row(p: &'static Project) -> impl IntoView {
    let ticks = p
        .ticks
        .iter()
        .map(|t| {
            view! {
                <span class="tick" class:live=*t == "live">
                    {*t}
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

    let link = p.link.map(|l| {
        view! {
            <a href=l.url target="_blank" rel="noopener noreferrer">
                {l.label}
                <span class="arr" aria-hidden="true">" ↗"</span>
            </a>
        }
    });

    view! {
        <article class="row">
            <p class="r-name">{p.name} {ticks} {tail}</p>
            <p class="r-line">{p.line} {link}</p>
            <p class="r-tech">{p.tech}</p>
        </article>
    }
}
