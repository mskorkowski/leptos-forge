//! Canvas component used for showing and manipulating a widget

use leptos::html::Div;
use leptos::prelude::*;

use reactive_stores::Store;

use crate::CanvasConfigurationStoreFields;
use crate::ForgeTheme;
use crate::LeptosForgeConfiguration;
use crate::LeptosForgeConfigurationStoreFields;
use crate::Story;

/// Canvas showing a component
#[component]
pub fn Canvas<UiStory>(
    /// story to be drawn
    story: UiStory,
    /// reference to canvas
    node_ref: NodeRef<Div>,
    /// store with user data
    data: Store<UiStory::Data>,
    /// leptos forge configuration
    configuration: Store<LeptosForgeConfiguration>,
) -> impl IntoView
where
    UiStory: 'static + Story + Copy,
{
    let view = story.view(data).into_any();

    let canvas_box_classes = "\
        leptos-forge-canvas-box \
        basis-2/3 \
        grow-1 \
        shrink-1 \
        justify-items-start \
        scrollbox \
        print:basis-full \
        print:flex-auto \
        print:overflow-visible \
        print:w-auto \
        print:h-auto \
    ";

    let canvas_classes = move || { 
        
        let canvas_background = match configuration.canvas().background().get() {
            None => "",
            Some(ForgeTheme::Light) => "light-canvas",
            Some(ForgeTheme::Dark) => "dark-canvas",
        };

        let base_classes= "\
            leptos-forge-canvas \
            scrollable \
            py-0 \
            m-4 \
            print:bg-white \
            print:overflow-visible \
            print:w-auto \
            print:h-auto \
            print:relative \
            isolate \
        ";


        format!("{base_classes} {canvas_background}")
    };

    view! {
        <div class=canvas_box_classes>
            <div class=canvas_classes node_ref=node_ref>
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
    /// store with user data
    data: Store<UiStory::Data>,
) -> impl IntoView
where
    UiStory: 'static + Story + Copy,
{
    let view = story.view(data).into_any();

    view! {
        <div class="leptos-forge-canvas relscrollable-100 m-4 bg-forgegray-100 min-h-25 h-max print:bg-white print:overflow-visible print:w-auto print:h-auto print:relative" node_ref=node_ref>
            { view }
        </div>
    }
}
