//! Controller code related to handling the select by the keyboard
//! 

use super::*;

/// Controller for [SingleSelect] which handles the keyboard logic
pub(super) struct KeyboardController;

impl KeyboardController {
    /// Handles the arrow down button when [SingleSelect] is focused
    /// 
    /// # Data states
    /// 
    /// 1. Single select is empty ([`value`][SingleSelectProps#id] is `None`) and there is nothing selected 
    ///    yet ([`model.selection`][SingleSelectModel#selection] is `None`) then we need to select first element
    ///    on the list.
    /// 2. Single select is empty ([`value`][SingleSelectProps#id] is `None`) and there is element which
    ///    is selected ([`model.selection`][SingleSelectModel#selection] is `Some`) then we need to select next
    ///    element on the list
    ///    1. If the selected element is the last on the list ([`model.selection.index`][Selection#index] equals 
    ///       [`model.items.len()-1`][SingleSelectModel#items]) then we can't advance the selection
    ///    
    pub(super) fn arrow_down<Value>(&self, model: Store<SingleSelectModel<Value>>, value: Signal<Option<Value>>)  
    where
        Value: Clone + Keyed + PatchField + ThreadSafe
    {
        
    }
}