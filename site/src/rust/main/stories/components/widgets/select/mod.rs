//! Stories for select widget
//! 

pub mod store_failure;

use std::fmt::Display;

use forge::RouteDef;
use leptos::prelude::*;
use reactive_stores::PatchField;
use reactive_stores::Store;
use ui_components::model::Keyed;
use ui_components::widgets::single_select::SingleSelect;
use ui_components::widgets::field::TextField;
use utils_leptos::signal::URwSignal;
use uuid::Uuid;

use forge::Story;

use crate::State;
use crate::stories::components::widgets::select::store_failure::ComponentWithAStoreStory;

/// Description of the label primitive
const WIDGET_DESC: &str = r############"
# SingleSelect
 
The `SingleSelect` widget allows selection of the single element from the list of options by showing a dropdown menu

"############;

/// Sample item
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Item {
    /// value of an item
    value: u32,
    /// key of an item
    key: Uuid,
}

impl PatchField for Item {
    fn patch_field(
        &mut self,
        new: Self,
        path: &reactive_stores::StorePath,
        notify: &mut dyn FnMut(&reactive_stores::StorePath),
        _keys: std::option::Option<&reactive_stores::KeyMap>
    ) {
        if *self != new{
            *self = new;
            notify(path);
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl Keyed for Item {
    fn key(&self) -> &Uuid {
        &self.key
    }
}

impl Item {
    /// Creates a new sample list of items to select from
    fn data_set() -> Vec<Self> {
        vec![
            Item{
                value: 1,
                key: Uuid::new_v4(),
            },
            Item{
                value: 2,
                key: Uuid::new_v4(),
            },
            Item{
                value: 3,
                key: Uuid::new_v4(),
            },
            Item{
                value: 4,
                key: Uuid::new_v4(),
            },
        ]
    }
}

/// story describing the basic label behavior
#[derive(Clone, Copy, Debug)]
pub struct BasicSingleSelectStory {
    /// Signal used to set the value of the label
    label: URwSignal<String>,
    /// Signal with currently selected value
    value: URwSignal<Option<Item>>,
}

impl Default for BasicSingleSelectStory {
    fn default() -> Self {
        let label = URwSignal::new("SingleSelect".to_string());
        let value: URwSignal<Option<Item>> = URwSignal::new(None);
        BasicSingleSelectStory{
            label,
            value
        }
    }
}


impl Story for BasicSingleSelectStory {
    type Data = State;

    fn view(&self, _state: Store<Self::Data>) -> impl IntoView {
        view! {
            <div class="relative">
                <SingleSelect
                    id="leptos-forge-1-select"
                    label={self.label}
                    value={self.value}
                    items={
                        Item::data_set()
                    }
                />
            </div>
        }
    }

    fn controls(&self, _state: Store<Self::Data>) -> impl IntoView {
        let label: URwSignal<String> = self.label;

        view! {
            <TextField text=label label="Alternative text" id="leptos-forge-2-alt-text"/> 
        }
    }

    fn description(&self) -> &'static str {
        WIDGET_DESC
    }

    fn subroutes(&self) -> Vec<RouteDef<Self::Data>> {
        vec![
            RouteDef::story::<ForceOpenSingleSelectStory>("force_open", "Force open"),
            RouteDef::story::<ComponentWithAStoreStory>("store_failure", "Store failure"),
        ]
    }
}

/// story describing the basic label behavior
#[derive(Clone, Copy, Debug)]
pub struct ForceOpenSingleSelectStory {
    /// Signal used to set the value of the label
    label: URwSignal<String>,
    /// Signal with currently selected value
    value: URwSignal<Option<Item>>,
}

impl Default for ForceOpenSingleSelectStory {
    fn default() -> Self {
        let label = URwSignal::new("SingleSelect".to_string());
        let value: URwSignal<Option<Item>> = URwSignal::new(None);
        ForceOpenSingleSelectStory{
            label,
            value
        }
    }
}


impl Story for ForceOpenSingleSelectStory {
    type Data = State;

    fn view(&self, _state: Store<Self::Data>) -> impl IntoView {
        view! {
            <div class="relative">
                <SingleSelect
                    id="leptos-forge-1-select"
                    label={self.label}
                    value={self.value}
                    items={
                        Item::data_set()
                    }
                />
            </div>
        }
    }

    fn controls(&self, _state: Store<Self::Data>) -> impl IntoView {
        let label: URwSignal<String> = self.label;

        view! {
            <TextField text=label label="Alternative text" id="leptos-forge-2-alt-text"/> 
        }
    }

    fn description(&self) -> &'static str {
        WIDGET_DESC
    }
}


