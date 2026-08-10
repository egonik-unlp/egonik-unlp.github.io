use leptos::prelude::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <header class="site-nav">
            <div class="container nav-inner">
                <a class="brand" href="#top">
                    <span class="brand-mark" aria-hidden="true">"EG/"</span>
                    <span class="brand-name">"field notes"</span>
                </a>
                <nav class="nav-links" aria-label="Primary">
                    <a href="#work"><span>"01"</span> "work"</a>
                    <a href="#skills"><span>"02"</span> "stack"</a>
                    <a href="#research"><span>"03"</span> "research"</a>
                    <a href="#contact"><span>"04"</span> "contact"</a>
                </nav>
            </div>
        </header>
    }
}
