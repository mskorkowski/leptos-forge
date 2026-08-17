//! Panel drawing the component and its controls so they can be adjusted

use leptos::prelude::*;

/// Panel for drawing the component and it's controls so the component can be adjusted
#[component]
pub fn ComponentPanel(
    /// elements in the component panel
    children: Children,
) -> impl IntoView {

    let class = "\
        leptos-forge-component-panel \
        basis-2/3 \
        grow-1 \
        shrink-1 \
        flex \
        flex-col \
        pt-4 \
        overflow-hidden \
        print:basis-full \
        print:flex-auto \
    ";

    view! {
        <div class=class>
            {children()}
        </div>
    }
}
