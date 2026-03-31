//! Module contains controller related to showing the floating menu for
//! [SingleSelect]

use crate::widgets::single_select::selection::SelectionController;

use super::*;

/// Controller for the floating ui element
pub(super) struct FloatingController;

impl FloatingController {
    /// Shows the floating menu
    pub(super) fn show<Value>(&self, model: Store<SingleSelectModel<Value>>)
    where
        Value: SingleSelectValue
    {
        let dropdown = model.dropdown();

        match dropdown.get_untracked() {
            DropdownState::Closed => 
                dropdown.patch(DropdownState::Open),
            DropdownState::ForceOpen | DropdownState::Open | DropdownState::ClickOpen=> {
                // it's already open
            }
        }
    }

    /// Hides the floating menu
    pub(super) fn hide<Value>(&self, model: Store<SingleSelectModel<Value>>)
    where
        Value: SingleSelectValue
    {
        let dropdown = model.dropdown();

        match dropdown.get_untracked() {
            DropdownState::ClickOpen | DropdownState::Open => {
                dropdown.patch(DropdownState::Closed)
            }
            DropdownState::Closed => {
                // already closed
            },
            DropdownState::ForceOpen => {
                // we never close this one
            }
        }
    }
}