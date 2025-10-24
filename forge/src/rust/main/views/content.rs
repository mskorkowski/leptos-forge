//! Content of the story

use leptos::prelude::*;

/// Content of the page
#[component]
pub fn Content(
    /// children components
    children: Children,
) -> impl IntoView {
    view! {
        <div class="leptos-forge-content divide-solid flex flex-row flex-1 overflow-hidden print:overflow-visible">
            { children() }
        </div>
    }
}
