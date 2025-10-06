//! Renders controls for the component rendered in the canvas

use leptos::prelude::*;

use crate::Story;

/// Control pane which is shown on the stories page
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

/// Control pane which is shown in embedded cases
#[component]
pub fn EmbeddedControlPane<UiStory>(
    /// story to be controlled
    story: UiStory
) -> impl IntoView 
where
    UiStory: 'static + Story + Copy,
{
    let view = story.controls().into_any();

    view! { 
        <div class="leptos-forge-control-pane p-4 relscrollable-100 min-h-30">
            { view }
        </div>
    }
}