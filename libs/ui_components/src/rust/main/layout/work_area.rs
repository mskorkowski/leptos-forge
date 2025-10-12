//! Module contains the work area under the toolbar

use leptos::prelude::*;

use super::content_pane::ContentPane;
use super::drawer::Drawer;

/// Component for the area to the left of the main menu
///
/// This is a place where user will focus his attention while working in the application
#[component]
pub fn WorkArea(
    /// content of the work area
    children: Children,
) -> impl IntoView {
    view! {
      <div class="w-full grow flex flex-row">
        <ContentPane children=children />
        <Drawer />
      </div>
    }
}
