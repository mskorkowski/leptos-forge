//! This module contains the data model for the [SingleSelect][super::SingleSelect]
//! widget
//! 

use leptos::leptos_dom::logging::console_log;
use reactive_stores::KeyMap;
use reactive_stores::Patch;
use reactive_stores::PatchField;
use reactive_stores::Store;
use reactive_stores::StorePath;
use utils_leptos::stores::stored_ref::StoredRef;

use crate::model::Key;
use crate::model::Keyed;

/// Item that can be selected in the single select widget
#[derive(Debug, Clone, Store, Patch)]
pub(super) struct SingleSelectItem<Value>
where
    Value: Clone + PatchField,
{
    /// value of the item
    pub(super) value: Value,
    /// reference to the node that represents the item
    pub(super) node_ref: StoredRef,
    /// id of the node 
    ///
    /// This value is used mainly by the accesibility part of the implementation
    pub(super) id: String,
}

impl<Value> Keyed for SingleSelectItem<Value>
where
    Value: Clone + Keyed + PatchField,
{
    fn key(&self) -> &Key {
        self.value.key()
    }
}

#[allow(clippy::to_string_trait_impl)]
impl<Value> ToString for SingleSelectItem<Value>
where
    Value: ToString + Clone + PatchField,
{
    fn to_string(&self) -> String {
        self.value.to_string()
    }
}

/// Selected element
#[derive(Debug, Clone, Store)]
pub(super) struct Selection{
    /// key of the selected element
    pub(super) key: Key,
    /// index in values
    pub(super) index: usize,
    /// node_ref of last selected element
    pub(super) node_ref: StoredRef,
    /// id of the last selected element
    /// 
    /// It's required for assistive technologies
    pub(super) id: String,
}

impl PatchField for Selection {
    fn patch_field(
            &mut self,
            new: Self,
            path: &StorePath,
            notify: &mut dyn FnMut(&StorePath),
            _keys: std::option::Option<&KeyMap>
    ) {
        console_log("Patching selection");
        if self.key != new.key {
            console_log("patch selection key changed");
            *self = new;
            notify(path);
        }
        else {
            console_log("patch selection key unchanged")
        }
    }
}

/// State of the dropdown menu
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Store)]
pub enum DropdownState {
    /// Opened by mouse event
    ClickOpen,
    /// Opened by other means that mouse event
    Open,
    /// Closed
    #[default]
    Closed,
    /// Forces dropdown to be opened
    ForceOpen,
}

impl DropdownState {
    /// Returns the next state of the popover
    ///
    /// | Current state | `click` | Result       |
    /// |:--------------|:-------:|:-------------|
    /// | ClickOpen     | any     | Open         |
    /// | Open          | any     | Closed       |
    /// | Closed        | true    | ClickOpen    |
    /// | Closed        | false   | Open         |
    /// | ForceOpen     | any     | ForceOpen    |
    pub fn toggle(self, click: bool) -> Self {
        match self {
            DropdownState::ClickOpen => DropdownState::Open,
            DropdownState::Open => DropdownState::Closed,
            DropdownState::Closed => if click {
                DropdownState::ClickOpen
            } else {
                DropdownState::Open
            }
            DropdownState::ForceOpen => DropdownState::ForceOpen,
        }
    }

    /// If this method returns `true` it means that popover
    /// should be visible
    pub fn is_open(&self) -> bool {
        *self != DropdownState::Closed
    }
}

impl PatchField for DropdownState {
    fn patch_field(
        &mut self,
        new: Self,
        path: &StorePath,
        notify: &mut dyn FnMut(&StorePath),
        _keys: std::option::Option<&KeyMap>
    ) {
        if *self != new {
            *self = new;
            notify(path);
        }
    }
}

/// Model for the single sel
#[derive(Store, Clone, Patch)]
pub(super) struct SingleSelectModel<Value>
where
    Value: Clone + Keyed + PatchField,
{
    /// id of the item with focus
    pub(super) selection: Option<Selection>,
    /// list of items to choose from
    #[store(key: Key = |counter| *counter.value.key())]
    pub(super) items: Vec<SingleSelectItem<Value>>,
    /// how many items are in the list
    pub(super) count: usize,
    /// state of the dropdown menu
    pub(super) dropdown: DropdownState,
}