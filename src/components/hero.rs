use leptos::prelude::*;

#[component]
pub fn Hero(spanish: bool) -> impl IntoView {
    let copy = if spanish {
        (
            "Eduardo Gonik · Ingeniero de datos e IA",
            "Construyo los ",
            "sistemas",
            " que transforman datos crudos en inteligencia útil.",
            "Diseño pipelines de embeddings, sistemas de predicción y los productos full-stack que los ponen en práctica.",
            "Explorar proyectos",
            "Empezar una conversación",
            "Señal actual",
            "enfoque",
            "recuperación · predicción · producto",
            "herramientas",
            "Rust · Python · Zig",
            "ubicación",
            "La Plata, Argentina",
            "Disponible para productos de datos ambiciosos",
        )
    } else {
        (
            "Eduardo Gonik · Data & AI Engineer",
            "Building the ",
            "systems",
            " between raw data and useful intelligence.",
            "I design embedding pipelines, prediction frameworks and the full-stack products that put them to work.",
            "Explore selected work",
            "Start a conversation",
            "Current signal",
            "focus",
            "retrieval · prediction · product",
            "toolkit",
            "Rust · Python · Zig",
            "base",
            "La Plata, Argentina",
            "Open to ambitious data products",
        )
    };

    view! {
        <section class="hero" id="top">
            <canvas class="lens-canvas" id="lens-canvas" aria-hidden="true"></canvas>
            <div class="hero-inner container">
                <div class="hero-copy">
                    <p class="hero-byline">{copy.0}</p>
                    <h1 class="hero-title">
                        {copy.1}
                        <span class="lit">{copy.2}</span>
                        {copy.3}
                    </h1>
                    <p class="hero-lede">{copy.4}</p>
                    <div class="hero-actions">
                        <a class="btn btn-primary" href="#work">{copy.5} <span aria-hidden="true">"↓"</span></a>
                        <a class="btn btn-ghost" href="#contact">{copy.6}</a>
                    </div>
                </div>
                <aside class="hero-signal" aria-label="Current profile">
                    <p class="signal-label">{copy.7}</p>
                    <dl>
                        <div><dt>{copy.8}</dt><dd>{copy.9}</dd></div>
                        <div><dt>{copy.10}</dt><dd>{copy.11}</dd></div>
                        <div><dt>{copy.12}</dt><dd>{copy.13}</dd></div>
                    </dl>
                    <p class="signal-status"><span aria-hidden="true"></span> {copy.14}</p>
                </aside>
            </div>
            <div class="hero-index mono" aria-hidden="true">"FIELD / 00—04"</div>
        </section>
    }
}
