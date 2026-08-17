//! Renders controls for the component rendered in the canvas

use leptos::prelude::*;
use reactive_stores::Store;
use utils::prelude::ThreadSafe;

use crate::Story;
use crate::LeptosForgeConfiguration;

/// Control pane which is shown on the stories page
#[component]
pub fn ControlPane<UiStory>(
    /// story to be controlled
    story: UiStory,
    /// store with user data
    data: Store<UiStory::Data>,
    /// leptos-forge state
    configuration: Store<LeptosForgeConfiguration>,
) -> impl IntoView
where
    UiStory: Story + Copy + ThreadSafe,
{
    view!{
        <div class="leptos-forge-control-pane-box basis-full scrollbox print:hidden">
            <div class="leptos-forge-control-pane p-4 scrollable isolate">
                { story.controls_with_forge_state(data, configuration).into_any() }
            </div>
        </div>
    }
}

/// Control pane which is shown in embedded cases
#[component]
pub fn EmbeddedControlPane<UiStory>(
    /// story to be controlled
    story: UiStory,
    /// store with user data
    data: Store<UiStory::Data>,
) -> impl IntoView
where
    UiStory: 'static + Story + Copy,
{
    let view = story.controls(data).into_any();

    view! {
        <div class="leptos-forge-control-pane p-4 relscrollable-100 min-h-30">
            { view }
        </div>
    }
}
