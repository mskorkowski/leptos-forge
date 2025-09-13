//! Component showing the description of the element
//! 

use leptos::prelude::*;
use ui_components::primitives::markdown::Markdown;

/// Description of the component shown in canvas
/// 
/// Text shown in description is a markdown formatted string.
#[component]
pub fn Description(
    /// Markdown formatted description of the component
    text: &'static str
) -> impl IntoView {
    view! {
        <div class="storybook-description flex flex-col basis-1/3 first:basis-1/1 px-4 py-4 overflow-auto print:hidden print:basis-0">
            <Markdown src={text} />
        </div>
    }
}