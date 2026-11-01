//! Main container of the application layout
//!

use leptos::prelude::*;

/// CSS classes for the root element
const ROOT_CSS_CLASSES: &str = "\
    leptos-forge-layout-root \
    w-dvw \
    h-dvh \
    flex \
    flex-col \
    py-0 \
    md:flex-row \
    print:w-auto \
    print:h-auto \
    print:overflow-visible
";

/// Root component of the application, using it allows seamless integration
/// with other layout components from this crate
#[component]
pub fn Root(children: Children) -> impl IntoView {
    view! {
        <div class=ROOT_CSS_CLASSES>
          { children() }
        </div>
    }
}
