// src/components/projects.rs — Syed Hassam Jan Portfolio × Leptos 0.7
//
// ── UI COMPONENT ─────────────────────────────────────────────────────────────
// Fully encapsulated component. Data is injected statically at compile time.
// ─────────────────────────────────────────────────────────────────────────────

use leptos::prelude::*;
use crate::content;

#[component]
pub fn Projects() -> impl IntoView {
    let projects = content::load().projects;
    view! {
        <section class="work-section" id="work">
            <div class="grid grid-cols-12 border-b">
                <div class="col-span-12 p-[clamp(16px,2.5vw,32px)] flex flex-col justify-center border-b">
                    <h2 class="section-heading font-sans text-[0.7rem] font-semibold tracking-[0.25em] uppercase text-muted">"PROJECTS"</h2>
                </div>
            </div>
            {projects.into_iter().map(|p| {
                let href = p.href.clone();
                view! {
                    <div
                        class="project-row invisible grid grid-cols-[auto_1fr] md:grid-cols-12 border-b"
                        data-accent=p.accent
                    >
                        <div class="col-span-1 p-[clamp(16px,2.5vw,32px)] border-r font-serif text-[0.9rem] font-bold text-red flex items-center justify-center">{p.num}</div>
                        <div class="col-span-1 md:col-span-8 p-[clamp(16px,2.5vw,32px)] flex flex-col justify-center gap-1.5 border-r">
                            <h3 class="serif-huge font-serif text-[clamp(2.5rem,5vw,4.5rem)] font-black leading-none tracking-[-0.02em]">{p.title}</h3>
                            <span class="text-[0.6rem] tracking-[0.2em] text-muted">{p.subtitle}</span>
                        </div>
                        <div class="hidden md:flex md:col-span-3 p-[clamp(16px,2.5vw,32px)] flex-col items-center justify-center">
                            <a
                                href=href
                                target="_blank"
                                class="hover-target inline-flex items-center gap-2 border border-current px-5 py-2.5 font-sans text-[0.65rem] font-semibold tracking-[0.15em] uppercase transition-colors hover:bg-bg-dark hover:text-text-inv"
                                style="text-decoration:none;"
                            >
                                "Live Link ↗"
                            </a>
                        </div>
                    </div>
                }
            }).collect_view()}
        </section>
    }
}