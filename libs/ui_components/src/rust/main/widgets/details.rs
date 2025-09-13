//! Summary/details like widget
//! 
//! It has been created to go around the limitations of the html's `details` and `summary` tags

use leptos::prelude::*;

/// Allows to represent the structure as the details widget
pub trait DetailsParts
 {
    /// Header component of the details widget
    fn summary(&self) -> AnyView;
    /// Body component of the details widget
    fn details(&self) -> AnyView;
}

impl DetailsParts for Box<dyn DetailsParts> {
    fn details(&self) -> AnyView {
        self.as_ref().details()
    }

    fn summary(&self) -> AnyView {
        self.as_ref().summary()
    }
}

/// Widget showing a summary and details
#[component]
pub fn Details<'details, D: DetailsParts>(
    /// The content of the details widget
    details: &'details D,
) -> impl IntoView {
    let header = details.summary();
    let body = details.details();
    view!{
        <div class="leptos-forge-details">
            <div class="leptos-forge-details-header">
                {header}
            </div>
            <div class="leptos-forge-details-body">
                {body}
            </div>
        </div>
    }
}