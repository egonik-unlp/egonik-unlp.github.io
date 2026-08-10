use leptos::prelude::*;

// TODO: replace with Eduardo's real LinkedIn URL.
const LINKEDIN_URL: &str = "https://www.linkedin.com/in/eduardo-gonik-bbb757133/";

#[component]
pub fn Contact(spanish: bool) -> impl IntoView {
    view! {
        <section id="contact" class="section contact">
            <div class="container narrow">
                <div class="section-head" data-reveal="">
                    <h2 class="section-title contact-title">
                        {if spanish { "Hablemos" } else { "Let's " }}
                        <span class="lit">{if spanish { "." } else { "talk." }}</span>
                    </h2>
                    <p class="section-lede">
                        {if spanish {
                            "Ingeniería de datos e IA, una colaboración o simplemente intercambiar ideas sobre Rust y bases de datos vectoriales: escribime."
                        } else {
                            "Data & AI engineering, a collaboration, or just comparing notes on Rust and vector databases — reach out."
                        }}
                    </p>
                </div>
                <ul class="contact-links" data-reveal="">
                    <li>
                        <a href="mailto:eduardogonik@gmail.com">
                            <span class="contact-key">{if spanish { "correo" } else { "email" }}</span>
                            <span class="contact-val">"eduardogonik@gmail.com"</span>
                            <span class="contact-arr" aria-hidden="true">"↗"</span>
                        </a>
                    </li>
                    <li>
                        <a href="https://github.com/egonik-unlp" target="_blank" rel="noopener noreferrer">
                            <span class="contact-key">"github"</span>
                            <span class="contact-val">"egonik-unlp"</span>
                            <span class="contact-arr" aria-hidden="true">"↗"</span>
                        </a>
                    </li>
                    <li>
                        <a href=LINKEDIN_URL target="_blank" rel="noopener noreferrer">
                            <span class="contact-key">"linkedin"</span>
                            <span class="contact-val">{if spanish { "conectar" } else { "connect" }}</span>
                            <span class="contact-arr" aria-hidden="true">"↗"</span>
                        </a>
                    </li>
                </ul>
            </div>
        </section>
    }
}
