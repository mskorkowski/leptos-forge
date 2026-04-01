//! Module contains controller related to showing the floating menu for
//! [SingleSelect]

use super::*;

/// Controller for the floating ui element
pub(super) struct FloatingController;

impl FloatingController {
    /// Shows the floating menu
    pub(super) fn show<Value>(&self, model: Store<SingleSelectModel<Value>>, click: bool)
    where
        Value: SingleSelectValue
    {
        let dropdown = model.dropdown();

        let state = dropdown.get_untracked();

        if !state.is_open() {
            dropdown.patch(state.toggle(click));
        }
        // else nothing to do
    }

    /// Hides the floating menu
    pub(super) fn hide<Value>(&self, model: Store<SingleSelectModel<Value>>)
    where
        Value: SingleSelectValue
    {
        let dropdown = model.dropdown();

        let state = dropdown.get_untracked();
        if state.is_open() {
            let new_state = DropdownState::Closed;
            dropdown.set(new_state)
        }
    }
}