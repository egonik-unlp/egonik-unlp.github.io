use leptos::prelude::*;

use crate::data::projects::{projects, Category, Project};

#[component]
pub fn Projects(spanish: bool) -> impl IntoView {
    let all = projects(spanish);

    let groups = Category::ordered()
        .into_iter()
        .map(|cat| {
            let rows = all
                .iter()
                .filter(|p| p.category == cat)
                .cloned()
                .enumerate()
                .map(|(i, p)| work_row(p, i, spanish))
                .collect::<Vec<_>>();
            view! {
                <div class="work-group">
                    <p class="group-label">{cat.label(spanish)}</p>
                    {rows}
                </div>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <section id="work" class="section">
            <div class="container">
                <div class="section-head" data-reveal="">
                    <h2 class="section-title">
                        {if spanish { "Proyectos " } else { "Selected " }}
                        <span class="lit">{if spanish { "seleccionados" } else { "work" }}</span>
                    </h2>
                    <p class="section-lede">
                        {if spanish {
                            "Un recorrido de punta a punta: desde la adquisición de datos y los pipelines de embeddings hasta un framework de predicción y las aplicaciones que lo llevan a producción."
                        } else {
                            "One end-to-end thread — from data acquisition and embedding pipelines to a prediction framework and the applications that ship it."
                        }}
                    </p>
                </div>
                {groups}
            </div>
        </section>
    }
}

fn work_row(p: Project, i: usize, spanish: bool) -> impl IntoView {
    let is_flag = p.title == "Lensing";
    let delay = format!("{}", i * 70);
    let number = format!("{:02}", i + 1);

    let links = p
        .links
        .iter()
        .map(|l| {
            view! {
                <a class="work-link" href=l.url target="_blank" rel="noopener noreferrer">
                    {if spanish && l.label == "Live Site" { "Sitio web" } else { l.label }}
                    <span class="arr" aria-hidden="true">" ↗"</span>
                </a>
            }
        })
        .collect::<Vec<_>>();

    let tech = p
        .tech
        .iter()
        .map(|t| view! { <li>{*t}</li> })
        .collect::<Vec<_>>();

    view! {
        <article class="work-row" class:is-flagship=is_flag data-reveal="" data-reveal-delay=delay>
            <span class="work-index" aria-hidden="true">{number}</span>
            <div class="work-head">
                <h3 class="work-name">{p.title}</h3>
                {p.private.then(|| view! { <span class="badge">{if spanish { "privado" } else { "private" }}</span> })}
                {links}
            </div>
            <div class="work-body">
                <p class="work-tagline">{p.tagline}</p>
                <p class="work-desc">{p.description}</p>
                <ul class="tech" aria-label=if spanish { "Tecnologías" } else { "Tech stack" }>{tech}</ul>
            </div>
        </article>
    }
}
