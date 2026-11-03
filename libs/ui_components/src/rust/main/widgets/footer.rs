//! Various footer kinds for the ui
//! 


use leptos::prelude::*;

use crate::model::ButtonClick;
use crate::model::ButtonColorSchema;
use crate::model::Kind;
use crate::model::Size;
use crate::primitives::footer::Footer as FooterPrimitive;

use super::button::*;

/// Footer with save and cancel
#[component]
pub fn SaveCancelButton(
    /// Cancel button click
    #[prop(into)]
    on_cancel_button: SignalSetter<ButtonClick>,
    /// Save button click
    #[prop(into)]
    on_save_button: SignalSetter<ButtonClick>,
    
) -> impl IntoView {
    let schema = ButtonColorSchema{
        outer: "",
        inner: "",
        border: "border-solid border-forgeblue-500 first:rounded-l-sm last:rounded-r-sm border-t-2 border-b-2 border-r-2 first:border-l-2 hover:bg-forgeblue-100",
        color: "text-forgegray-800",
        background: "bg-forgeblue-300"
    };

    view!{
        <FooterPrimitive>
            <Button
                click=on_cancel_button
                style={Kind::Secondary.color(schema) + Size::Big}
            >Cancel</Button>
            <Button
                click=on_save_button
                style={Kind::Primary.color(schema)  + Size::Big}
            >Save</Button>
        </FooterPrimitive>
    }
}