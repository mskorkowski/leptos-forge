//! Module holds the selection controller for the [SingleSelect]
//! component

use std::fmt::Debug;

use leptos::prelude::*;
use utils_leptos::element::use_scroll_into_view;
use super::*;

use reactive_stores::Patch;
use reactive_stores::Store;

use crate::model::Keyed;
use crate::widgets::single_select::model::Selection;
use crate::widgets::single_select::model::SingleSelectModel;
use crate::widgets::single_select::model::SingleSelectModelStoreFields;

/// Selection controller
#[derive(Debug, Clone, Copy)]
pub(super) struct SelectionController;

impl SelectionController {
    /// Selects the next element from the item list
    /// 
    /// - If no element is selected 
    ///   - And list has items then it selects first element from the list
    ///   - And list is empty then nothing happens
    /// - If element is selected
    ///   - And it's last element of the list then nothing happens
    ///   - And it's not last element on the list selects next element
    pub(super) fn select_next<Value>(&self, model: Store<SingleSelectModel<Value>>)
    where 
        Value: SingleSelectValue,
    {
        let items: Vec<SingleSelectItem<Value>> = model.items().get_untracked();
        let selection = match model.selection().get_untracked() {
            None => {
                if !items.is_empty() { // if there is any element in the list
                                        // select first
                    let first = &items[0];
                    let selection = Selection{
                        key: *first.key(),
                        index: 0,
                        node_ref: first.node_ref.clone(),
                        id: first.id.clone()
                    };
                    Some(selection)
                }
                else {
                    None
                }
            }        
            Some(old_selection) => {
                if items.len() > old_selection.index + 1 {
                    let next = &items[old_selection.index + 1];
                    let selection = Selection {
                        key: *next.key(),
                        index: old_selection.index + 1,
                        node_ref: next.node_ref.clone(),
                        id: next.id.clone()
                    };
                    self.mark_unselected(&old_selection.node_ref);
                    Some(selection)

                }
                else {
                    None
                }
            }
        };

        if let Some(selection) = selection {
            self.mark_selected(&selection.node_ref);
            model.selection().patch(Some(selection))
        }
    }

    /// Selects previous element from the list
    /// 
    /// - If no element was selected select first element
    /// - If there is selected element
    ///   - If it's firs element on the list - keep the selection
    ///   - If it's not the first element on the list select one before
    pub(super) fn select_prev<Value>(&self,  model: Store<SingleSelectModel<Value>>)
    where
        Value: SingleSelectValue
    {
        let items = model.items().get_untracked();
        let selection = match model.selection().get_untracked() {
            None => {
                if !items.is_empty() { // if there is any element in the list
                                        // select first
                    let first = &items[0];
                    let selection = Selection{
                        key: *first.key(),
                        index: 0,
                        node_ref: first.node_ref.clone(),
                        id: first.id.clone()
                    };
                    Some(selection)
                }
                else {
                    None
                }
            }        
            Some(old_selection) => {
                if old_selection.index > 0 {
                    let next = &items[old_selection.index - 1];
                    let selection = Selection {
                        key: *next.key(),
                        index: old_selection.index - 1,
                        node_ref: next.node_ref.clone(),
                        id: next.id.clone()
                    };
                    self.mark_unselected(&old_selection.node_ref);
                    Some(selection)

                }
                else {
                    None
                }
            }
        };

        if let Some(selection) = selection {
            self.mark_selected(&selection.node_ref);

            console_log(&format!("Selected element at: {}", selection.index));

            model.selection().patch(Some(selection))
        }
    }

    /// Updates the value of the [SingleSelect] component to be the currently selected one
    pub(super) fn select<Value>(&self, model: Store<SingleSelectModel<Value>>, value: URwSignal<Option<Value>>) 
    where
        Value: SingleSelectValue
    {
        model.items().with_untracked(|items| {
            let selection = model.selection().get_untracked();

            if let Some(selection) = selection &&
                items.len() > selection.index 
            {
                value.set(
                    Some(
                        items[selection.index].value.clone()
                    )
                );
            }
        });
    }

    /// Apply the style of unselected element
    pub(super) fn mark_unselected(&self, node_ref: &StoredRef) {
        use_remove_class(node_ref, ("bg-forgeblue-300", "bg-forgeblue-200"));
    }
    
    /// Apply the style of selected element
    pub(super) fn mark_selected(&self, node_ref: &StoredRef) {
        use_swap_class(node_ref, "bg-forgeblue-300", "bg-forgeblue-200");
        use_scroll_into_view(node_ref);
    }
}