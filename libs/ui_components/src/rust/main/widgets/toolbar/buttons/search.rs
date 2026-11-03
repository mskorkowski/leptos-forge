//! Contains implementation of the search button
//! 


use leptos::prelude::component;
use leptos::prelude::IntoView;
use leptos::prelude::Signal;
use leptos::prelude::SignalSetter;
use leptos::prelude::view;
use utils_leptos::signal::URwSignal;

use crate::model::ButtonClick;
use crate::widgets::button::Button;
use crate::widgets::field::TextField;

use super::super::SINGLE_BUTTON_STYLE;



/// Search button menu item
/// 
/// This component depends on the [ToolbarContainer] to provide the html/css
/// environment
#[component]
pub fn SearchMenuItem(
    /// signal with the toolbar state
    #[prop(into)]
    click_in: SignalSetter<ButtonClick>,
) -> impl IntoView
{

    // let (click_out, click_in) = signal(ButtonClick::Released);
    view!{
        <Button
            click={click_in}
            style=SINGLE_BUTTON_STYLE
        >
            <leptos_icons::Icon icon=icondata::MdiMagnify />
        </Button>
    }
}

/// Basic text search input field
/// 
/// When search button is pressed it will show a search input field replacing the contents
/// of the toolbar
#[component]
pub fn SearchInput<S1: ToString>(
    /// Id of the search field
    id: S1,
    /// Input of the search field
    #[prop(into)]
    input: URwSignal<String>,
    /// Hint for the search field
    #[prop(into)]
    label: Signal<String>,
) -> impl IntoView {
    view!{
        <TextField
            id
            text=input
            label
        />
    }
}