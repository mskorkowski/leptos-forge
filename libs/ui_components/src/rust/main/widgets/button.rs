//! Various kinds of button widgets to be used


use leptos::prelude::*;

use crate::model::ButtonClick;
use crate::model::ButtonGroup;
use crate::model::Style;

pub use super::super::primitives::button::Button as PrimitiveButton;




/// Button widget
#[component]
pub fn Button<
    Id: ToString,
>(
    /// Id of the button
    id: Id,
    /// Signal which triggers on clicks
    /// Signal triggered when the button is clicked
    #[prop(into)]
    click: SignalSetter<ButtonClick>,
    /// kind of the button
    #[prop(into, name="style")]
    style: Style,
    /// Content of the button
    #[prop(optional, default=Box::new(|| view!{"Press me!"}.into_any()))]
    children: Children,
    /// Button group
    #[prop(optional, default=None)]
    group: Option<ButtonGroup>,
) -> impl IntoView {
    view!{
        <PrimitiveButton<Id>
            id
            click
            group
            children
            style
        />
    }
}