//! Simple toolbar example
 

use leptos::prelude::*;
use leptos_icons::Icon;
use reactive_stores::Store;

use forge::Story;
use utils::prelude::*;
use ui_components::widgets::toolbar::Toolbar;
use ui_components::widgets::toolbar::ToolbarItemView;

use crate::State;


/// Search button description
const DESCRIPTION_TOOLBAR_WITH_SEARCH: &str =  r############"
# Bottom navigation toolbar
# Simple

## Placement

- Should be placed at the bottom of the screen for mobile devices

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

impl Enumerable for MenuItems {
    fn iter() -> impl Iterator<Item = Self> {
        vec![
            MenuItems::Home,
            MenuItems::Tasks,
            MenuItems::Emails
        ].into_iter()
    }
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

/// Story for toolbar with search
#[derive(Debug, Clone, Copy, Default)]
pub struct SimpleToolbarStory {}

impl Story for SimpleToolbarStory {
    type Data = State;

    fn description(&self) -> &'static str {
        DESCRIPTION_TOOLBAR_WITH_SEARCH
    }

    fn view(&self, _data: Store<Self::Data>) -> impl IntoView {
        
        view!{
            <Toolbar<MenuItems> 
                search=MenuItems::Search
            />
        }

    }
}