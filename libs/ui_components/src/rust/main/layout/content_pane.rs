//! Module contains the content pane code

use leptos::prelude::*;

/// Area where the user requested content will be placed into
#[component]
pub fn ContentPane(
    /// content of the content pane
    children: Children,
) -> impl IntoView {
    view! {
      <div class="w-full grow flex flex-row bg-forgebrown-100">
        { children() }
      </div>
    }
}
