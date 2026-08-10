use leptos::prelude::*;

#[component]
pub fn Nav(spanish: bool) -> impl IntoView {
    let (work, stack, research, contact, switch_href, switch_label, switch_lang) = if spanish {
        (
            "proyectos",
            "herramientas",
            "investigación",
            "contacto",
            "/",
            "EN",
            "en",
        )
    } else {
        ("work", "stack", "research", "contact", "/es", "ES", "es")
    };

    view! {
        <header class="site-nav">
            <div class="container nav-inner">
                <a class="brand" href="#top">
                    <span class="brand-mark" aria-hidden="true">"EG/"</span>
                    <span class="brand-name">"field notes"</span>
                </a>
                <nav class="nav-links" aria-label="Primary">
                    <a href="#work"><span>"01"</span> {work}</a>
                    <a href="#skills"><span>"02"</span> {stack}</a>
                    <a href="#research"><span>"03"</span> {research}</a>
                    <a href="#contact"><span>"04"</span> {contact}</a>
                    <a class="language-switch" href=switch_href lang=switch_lang>{switch_label}</a>
                </nav>
            </div>
        </header>
    }
}
