// src/components/services.rs — Syed Hassam Jan Portfolio × Leptos 0.7
//
// ── UI COMPONENT ─────────────────────────────────────────────────────────────
// Replaces insights.rs — displays the list of services offered.
// ─────────────────────────────────────────────────────────────────────────────

use leptos::prelude::*;
use crate::content;

#[component]
pub fn Services() -> impl IntoView {
    let services = content::load().services;
    view! {
        <section class="insights-section">
            <div class="grid grid-cols-12 border-b">
                <div class="col-span-12 p-[clamp(16px,2.5vw,32px)] flex flex-col justify-center border-b">
                    <h2 class="section-heading font-sans text-[0.7rem] font-semibold tracking-[0.25em] uppercase text-muted">"SERVICES"</h2>
                </div>
            </div>
            {services.into_iter().enumerate().map(|(i, s)| {
                let num = format!("{:02}", i + 1);
                view! {
                    <div class="article-row grid grid-cols-[auto_1fr] md:grid-cols-12 border-b">
                        <div class="col-span-1 md:col-span-2 p-[clamp(16px,2.5vw,32px)] border-r text-[0.6rem] font-semibold tracking-[0.2em] text-red flex items-center">{num}</div>
                        <div class="col-span-1 md:col-span-10 p-[clamp(16px,2.5vw,32px)] font-serif text-[clamp(1.2rem,2.5vw,2rem)] font-bold flex items-center">{s.name}</div>
                    </div>
                }
            }).collect_view()}
        </section>
    }
}
