//! Controller code related to handling the select by the keyboard
//! 

use crate::widgets::single_select::float::FloatingController;
use crate::widgets::single_select::selection::SelectionController;

use super::*;

/// Controller for [SingleSelect] which handles the keyboard logic
#[derive(Debug, Clone, Copy)]
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
    pub(super) fn arrow_down<Value>(&self, model: Store<SingleSelectModel<Value>>, _value: Signal<Option<Value>>)  
    where
        Value: SingleSelectValue
    {
        FloatingController.show(model, false);
        SelectionController.select_next(model);
    }

    /// Handles the arrow up button when [SingleSelect] is focused
    /// 
    /// # Data states
    /// 
    /// 1. Single select is empty ([`value`][SingleSelectProps#id] is `None`) and there is nothing selected 
    ///    yet ([`model.selection`][SingleSelectModel#selection] is `None`) then we need to select first element
    ///    on the list.
    /// 2. Single select is empty ([`value`][SingleSelectProps#id] is `None`) and there is element which
    ///    is selected ([`model.selection`][SingleSelectModel#selection] is `Some`) then we need to select next
    ///    element on the list
    ///    1. If the already selected element is the first on the list ([`model.selection.index`][Selection#index] equals 
    ///       0) then we can't advance the selection
    ///    
    pub(super) fn arrow_up<Value>(&self, model: Store<SingleSelectModel<Value>>, _value: Signal<Option<Value>>)  
    where
        Value: SingleSelectValue
    {
        FloatingController.show(model, false);
        SelectionController.select_prev(model);
    }

    /// Handles pressing enter
    /// 
    /// This method handles also <kbd>`left arrow`</kbd>, <kbd>`right arrow`</kbd>, <kbd<`space`</kbd>
    pub(super) fn enter<Value>(&self, model: Store<SingleSelectModel<Value>>, value: URwSignal<Option<Value>>)
    where
        Value: SingleSelectValue
    {
        SelectionController.select(model, value);
        FloatingController.hide(model);
    }

    /// Hides the floating menu without changing the state
    pub(super) fn escape<Value>(&self, model: Store<SingleSelectModel<Value>>,) 
    where
        Value: SingleSelectValue
    {
        FloatingController.hide(model);
    }
}