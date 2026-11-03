//! Bottom navigation toolbar search button
//! 

use forge::Story;
use reactive_stores::Store;
use ui_components::model::ButtonClick;
use ui_components::widgets::toolbar::SearchMenuItem;
use ui_components::widgets::toolbar::ToolbarContainer;
use ui_components::widgets::toolbar::ToolbarItemView;
use utils::prelude::Enumerable;
use utils_leptos::signal::URwSignal;

use crate::State;

use leptos::prelude::*;

/// Search button description
const DESCRIPTION_SEARCH_BUTTON: &str =  r############"
# Bottom navigation toolbar
# Search button

## Placement

- Should be placed at the right of the bottom navigation toolbar whenever possible
(there is space).
- If there is no space should be at the right top corner

## Search component

Clicking the button activates the search component.

"############;


/// Menu items for the search menu story
#[derive(Debug, Clone, Copy, Default, Store)]
enum MenuItems {
    /// Value of the search signal
    #[default]
    Search
}

impl ToolbarItemView for MenuItems {
    fn icon(&self) -> Option<AnyView> {
        use MenuItems::*;
        match self {
            Search => None
        }
    }

    fn label(&self) -> Option<AnyView> {
        use MenuItems::*;
        match self {
            Search => None
        }
    }

    fn id(&self) -> impl ToString {
        use MenuItems::*;

        match self {
            Search => "example-toolbar-search_button-search",
        }
    }
}

impl Enumerable for MenuItems {
    fn iter() -> impl Iterator<Item = Self> {
        vec![].into_iter()
    }
}

/// Story for the search button
#[derive(Debug, Clone, Copy, Default, Store)]
pub struct SearchButtonStory {
    /// signal tracking selection of items in the menu
    state: URwSignal<ButtonClick>,
}

impl Story for SearchButtonStory {
    type Data = State;

    fn description(&self) -> &'static str {
        DESCRIPTION_SEARCH_BUTTON
    }

    fn view(&self, _data: Store<Self::Data>) -> impl IntoView {
        
        let state: SignalSetter<ButtonClick> = self.state.write_only();

        view!{
            <ToolbarContainer>
                <SearchMenuItem
                    click_in=state
                />
            </ToolbarContainer>
        }

    }
}