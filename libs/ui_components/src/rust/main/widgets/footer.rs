//! Various footer kinds for the ui
//! 


use leptos::prelude::*;

use crate::primitives::footer::Footer as FooterPrimitive;

use super::button::*;

/// Footer with save and cancel
#[component]
pub fn SaveCancelButton<Id: ToString>(
    /// Id of the footer
    id: Id,
    /// Cancel button click
    #[prop(into)]
    on_cancel_button: SignalSetter<ButtonClick>,
    /// Save button click
    #[prop(into)]
    on_save_button: SignalSetter<ButtonClick>,
    
) -> impl IntoView {
    let id = id.to_string();
    let cancel_id: String = format!("{}-cancel", id);
    let save_id: String = format!("{}-save", id);

    let schema = ColorSchema{
        border: "border-1 border-solid border-forgeblue-500",
        color: "fg-forgegray-800",
        background: "bg-forgeblue-300"
    };

    view!{
        <FooterPrimitive>
            <Button
                id=cancel_id
                click=on_cancel_button
                style={Kind::Secondary.color(schema) + Size::Big}
            >Cancel</Button>
            <Button
                id=save_id
                click=on_save_button
                style={Kind::Primary.color(schema)  + Size::Big}
            >Save</Button>
        </FooterPrimitive>
    }
}