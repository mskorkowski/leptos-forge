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
    view! { 
        <div class="storybook-control-pane-box basis-1/3 flex-none scrollbox print:hidden">
            <div class="storybook-control-pane p-4 scrollable">
                { story.controls() }
            </div>
        </div>
    }
}