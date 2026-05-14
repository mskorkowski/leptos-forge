//! Stories for select widget
//! 

use std::fmt::Display;

use forge::RouteDef;
use leptos::prelude::*;
use reactive_stores::PatchField;
use reactive_stores::Store;
use ui_components::model::Key;
use ui_components::model::Keyed;
use ui_components::widgets::single_select::SingleSelect;
use ui_components::widgets::single_select::DropdownState;
use ui_components::widgets::field::TextField;
use ui_components::widgets::single_select::SingleSelectItemView;
use utils_leptos::signal::URwSignal;

use forge::Story;

use crate::State;

/// Description of the label primitive
const WIDGET_DESC: &str = r############"
# SingleSelect
 
The `SingleSelect` widget allows selection of the single element from the list of options by showing a dropdown menu

## Usage

Use it when user should select the value from one of the items from the list.

## Rendering items

Item's on the list should implement the `SingleSelectItemView` trait. 

```rust
/// Trait which needs to be implemented by the `Value` type so it can be displayed 
/// using the [SingleSelect]
pub trait SingleSelectItemView{
    /// Method is called for every item, so it can display itself on the selection
    /// list
    fn selection_list_view(self) -> impl IntoView;

    /// Method is called to show the selected item
    /// 
    /// ## Default implementation
    /// 
    /// By default it returns the same view as [`selection_list_view`][SingleSelectItemView::selection_list_view]
    fn selected_item_view(self) -> impl IntoView;
}
```

When user opens the dropdown menu, then items on the list are rendered using the
`SingleSelectItemView::selection_list_view`

Selected item is rendered using the `SingleSelectItemView::selected_item_view`.

If you need to listen to any events inside your custom components you must 
stop event propagation, otherwise default behavior of the `SingleSelect` component 
will trigger.

## Behavior

Component tries to replicate as much as possible the behavior of the html `<select>` tag.

- click to open and then click to select
- mouse down and drag the mouse over item to be selected and mouse up to select

Component enhances the `<select>` tag with

- custom item rendering both on the dropdown menu and selected item
- adds clear selection button

## Differences between the browsers

In Firefox when using mouse down + drag + mouse up to select the item which is
above the mouse is not highlighted since Firefox doesn't trigger the `:over`
pseudoclass in such case.

"############;

/// Sample item
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Item {
    /// value of an item
    value: u32,
    /// key of an item
    key: Key,
}

impl SingleSelectItemView for Item {
    fn selection_list_view(self) -> impl IntoView {
        view!{
            {self.value.to_string()}
        }
    }
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
    fn key(&self) -> &Key {
        &self.key
    }
}

impl Item {
    /// Creates a new sample list of items to select from
    fn data_set() -> Vec<Self> {
        vec![
            Item{
                value: 1,
                key: Key::random(),
            },
            Item{
                value: 2,
                key: Key::random(),
            },
            Item{
                value: 3,
                key: Key::random(),
            },
            Item{
                value: 4,
                key: Key::random(),
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
                    initial_state={DropdownState::ForceOpen}
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


