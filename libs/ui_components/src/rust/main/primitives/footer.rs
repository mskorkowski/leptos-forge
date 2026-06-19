//! Primitive to organize the footer of the component to be used with forms
//! or modals

use leptos::prelude::*;

/// Footer of the form or modal dialog
#[component]
pub fn Footer(
    /// Children of the footer
    children: Children

) -> impl IntoView {
    view!{
        <div 
            class="w-full justify-items-end-safe text-end border-t-1 border-forgeblue-500 mt-6 pt-3"
        >
            {children()}
        </div>
    }
}