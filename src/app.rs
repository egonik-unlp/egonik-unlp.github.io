use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::StaticSegment;

use crate::components::contact::Contact;
use crate::components::hero::Hero;
use crate::components::nav::Nav;
use crate::components::projects::Projects;
use crate::components::research::Research;
use crate::components::skills::Skills;

/// The server-rendered HTML document shell.
pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <meta
                    name="description"
                    content="Eduardo Gonik — Data & AI Engineer. Embedding pipelines, vector search, prediction frameworks and the full-stack apps that serve them."
                />
                <meta name="theme-color" content="#12121a"/>
                <link rel="icon" href="/favicon.svg" type="image/svg+xml"/>
                <script>"document.documentElement.classList.add('js')"</script>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
                <script src="/js/hero-lens.js" defer></script>
                <script src="/js/interactions.js" defer></script>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/website.css"/>
        <Title text="Eduardo Gonik — Data & AI Engineer"/>

        <Router>
            <Routes fallback=|| view! { <p class="container">"Page not found."</p> }>
                <Route path=StaticSegment("") view=HomePage/>
            </Routes>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Nav/>
        <main>
            <Hero/>
            <Skills/>
            <Projects/>
            <Research/>
        </main>
        <Contact/>
        <footer class="site-footer">
            <div class="container footer-inner">
                <span>"Eduardo Gonik / Data & AI Engineering"</span>
                <span>"Built in Rust · 2026"</span>
            </div>
        </footer>
    }
}
