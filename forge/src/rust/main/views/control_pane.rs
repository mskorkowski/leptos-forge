//! Renders controls for the component rendered in the canvas

use leptos::prelude::*;

use crate::Story;

#[component]
pub fn ControlPane<UiStory>(
    /// story to be controlled
    story: UiStory
) -> impl IntoView 
where
    UiStory: 'static + Story + Copy,
{
    let view = story.controls().into_any();

    view! { 
        <div class="leptos-forge-control-pane-box basis-1/3 flex-none scrollbox print:hidden">
            <div class="leptos-forge-control-pane p-4 scrollable">
                { view }
            </div>
        </div>
    }
}