//! Canvas component used for showing and manipulating a widget

use leptos::html::Div;
use leptos::prelude::*;

use crate::Story;

/// Canvas showing a component
#[component]
pub fn Canvas<UiStory>(
    /// story to be drawn
    story: UiStory,
    /// reference to canvas
    node_ref: NodeRef<Div>,
) -> impl IntoView 
where
    UiStory: 'static + Story + Copy,
{
    let view = story.view().into_any();

    view!{
        <div class="leptos-forge-canvas-box basis-2/3 flex-none justify-items-start scrollbox print:basis-full print:flex-auto print:overflow-visible print:w-auto print:h-auto">
            <div class="leptos-forge-canvas scrollable m-4 bg-forgegray-100 print:bg-white print:overflow-visible print:w-auto print:h-auto print:relative" node_ref=node_ref>
                { view }
            </div>
        </div>
    }
}

/// Canvas showing a component used for embedded context
#[component]
pub fn EmbeddedCanvas<UiStory>(
    /// story to be drawn
    story: UiStory,
    /// reference to canvas
    node_ref: NodeRef<Div>,
) -> impl IntoView 
where
    UiStory: 'static + Story + Copy,
{
    let view = story.view().into_any();

    view!{
        <div class="leptos-forge-canvas relscrollable-100 m-4 bg-forgegray-100 min-h-25 h-max print:bg-white print:overflow-visible print:w-auto print:h-auto print:relative" node_ref=node_ref>
            { view }
        </div>
    }
}