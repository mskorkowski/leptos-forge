//! Panel drawing the component and its controls so they can be adjusted

use leptos::prelude::*;

/// Panel for drawing the component and it's controls so the component can be adjusted
#[component]
pub fn ComponentPanel(
    /// elements in the component panel
    children: Children
) -> impl IntoView {
    view! { 
        <div class="leptos-forge-component-panel basis-2/3 flex flex-col flex-none pt-4 overflow-hidden print:basis-full print:flex-auto">
            {children()}
        </div>
    }
}