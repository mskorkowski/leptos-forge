//! Main container of the application layout
//! 

use leptos::prelude::*;

/// Root component of the application, using it allows seamless integration
/// with other layout components from this crate
#[component]
pub fn Root(
    
    children: Children,
) -> impl IntoView {
    view!{
        <div class="leptos-forge-layout-root w-dvw h-dvh flex flex-col sm:flex-row py-0 print:w-auto print:h-auto print:overflow-visible">
          { children() }
        </div>
    }
}