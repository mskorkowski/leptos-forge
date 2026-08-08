//! Contains model for the visibility toggling

/// Defines possible states of visibility of the component
#[derive(Debug, Clone, Copy, PartialEq)]
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