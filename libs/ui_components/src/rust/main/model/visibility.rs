//! Contains model for the visibility toggling

use reactive_stores::PatchField;
use reactive_stores::Store;

/// Defines possible states of visibility of the component
#[derive(Debug, Clone, Copy, PartialEq, Store)]
pub enum Visibility{
    /// Notify the component it should show itself
    Show,
    /// Set by the component when it's visible
    Visible,
    /// Notify the component it should hide itself
    Hide,
    /// Set by the component when it's hidden
    Hidden,
}

impl PatchField for Visibility {
    fn patch_field(
        &mut self,
        new: Self,
        path: &reactive_stores::StorePath,
        notify: &mut dyn FnMut(&reactive_stores::StorePath),
        _keys: Option<&reactive_stores::KeyMap>,
    ) {
        if *self != new {
            *self = new;
            notify(path)
        }
    }
}