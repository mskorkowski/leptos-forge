//! Module contains the code of the menu

use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;

/// Vertical main menu component on the left hand side
#[component]
pub fn MainMenu(
    /// Child elements of the main menu
    children: Children,
) -> impl IntoView {
    console_log("Rendering <App>");
    view! {
      <div class="flex-none flex-none py-4 px-4 flex flex-col bg-forgeblue-950 text-forgegray-200 scrollbox w-61 basis-auto print:hidden forge-text-standard">
        <div class="scrollable">
          { children() }
        </div>
      </div>
    }
}
