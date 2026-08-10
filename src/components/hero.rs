use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="hero" id="top">
            <canvas class="lens-canvas" id="lens-canvas" aria-hidden="true"></canvas>
            <div class="hero-inner container">
                <div class="hero-copy">
                    <p class="hero-byline">"Eduardo Gonik · Data & AI Engineer"</p>
                    <h1 class="hero-title">
                        "Building the "
                        <span class="lit">"systems"</span>
                        " between raw data and useful intelligence."
                    </h1>
                    <p class="hero-lede">
                        "I design embedding pipelines, prediction frameworks and the full-stack \
                         products that put them to work."
                    </p>
                    <div class="hero-actions">
                        <a class="btn btn-primary" href="#work">"Explore selected work" <span aria-hidden="true">"↓"</span></a>
                        <a class="btn btn-ghost" href="#contact">"Start a conversation"</a>
                    </div>
                </div>
                <aside class="hero-signal" aria-label="Current profile">
                    <p class="signal-label">"Current signal"</p>
                    <dl>
                        <div><dt>"focus"</dt><dd>"retrieval · prediction · product"</dd></div>
                        <div><dt>"toolkit"</dt><dd>"rust · python · zig"</dd></div>
                        <div><dt>"base"</dt><dd>"la plata, argentina"</dd></div>
                    </dl>
                    <p class="signal-status"><span aria-hidden="true"></span> "Open to ambitious data products"</p>
                </aside>
            </div>
            <div class="hero-index mono" aria-hidden="true">"FIELD / 00—04"</div>
        </section>
    }
}
