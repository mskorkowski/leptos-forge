//! Bottom navigation toolbar search button
//! 

use forge::Story;
use leptos_icons::Icon;
use reactive_stores::Store;
use ui_components::widgets::toolbar::Toolbar;
use ui_components::widgets::toolbar::ToolbarItemView;
use utils::prelude::Enumerable;

use crate::State;

use leptos::prelude::*;

/// Search button description
const DESCRIPTION_SEARCH_BUTTON: &str =  r############"
# Bottom navigation toolbar
# Toolbar with search button

## Placement

Toolbar should be placed at the bottom of the screen for small mobile devices
(phones) and on top for bigger one (tablets).

## Search component

Clicking the button activates the search component.

"############;


/// Menu items for the search menu story
#[derive(Debug, Clone, Copy)]
enum MenuItems {
    /// Home page
    Home,
    /// List of tasks
    Tasks,
    /// Emails
    Emails,
    /// Search
    /// 
    /// This item is not returned from the Enumerable implementation
    Search
}

impl ToolbarItemView for MenuItems {
    fn icon(&self) -> impl IntoView {
        use MenuItems::*;
        match self {
            Home => view!{
                <Icon icon=icondata::MdiHome />
            }.into_any(),
            Tasks => view!{
                <Icon icon=icondata::MdiFormatListChecks />
            }.into_any(),
            Emails => view!{
                <Icon icon=icondata::MdiEmailOutline />
            }.into_any(),
            Search => ().into_any()
        }
    }

    fn label(&self) -> impl IntoView {
        use MenuItems::*;
        match self {
            Home => "Home".into_any(),
            Tasks => "Tasks".into_any(),
            Emails => "Emails".into_any(),
            Search => ().into_any()
        }
    }

    fn id(&self) -> impl ToString {
        use MenuItems::*;

        match self {
            Home => "example-toolbar-simple-home",
            Tasks => "example-toolbar-simple-tasks",
            Emails => "example-toolbar-simple-emails",
            Search => "example-toolbar-simple-search",
        }
    }
}

impl Enumerable for MenuItems {
    fn iter() -> impl Iterator<Item = Self> {
        vec![
            MenuItems::Home,
            MenuItems::Tasks,
            MenuItems::Emails
        ].into_iter()
    }
}

/// Story for the search button
#[derive(Debug, Clone, Copy, Default)]
pub struct ToolbarWithSearchStory {

}

impl Story for ToolbarWithSearchStory {
    type Data = State;

    fn description(&self) -> &'static str {
        DESCRIPTION_SEARCH_BUTTON
    }

    fn view(&self, _data: Store<Self::Data>) -> impl IntoView {
        
        view!{
            <Toolbar<MenuItems> 
                search=MenuItems::Search
            />
        }

    }
}