//! Module contains code with a primitive structures to create a button
//! allowing file download
//! 
//! It must be different from other buttons sice file download is only
//! possible via link action

use leptos::ev::MouseEvent;
use leptos::ev::PointerEvent;
use leptos::prelude::*;
use base64::{Engine as _, engine::general_purpose};

use crate::model::ButtonClick;
use crate::model::Style;

/// Button allowing download operation
/// 
/// `DownloadButton` follows the [Button][super::Button] in terms of the
/// behavior and styling as close as possible.
#[component]
pub fn DownloadButton(
    /// Content of the file
    #[prop(into)]
    file_content: Signal<String>,
    /// Mime type of the file
    #[prop(into)]
    mime_type: Signal<String>,
    /// Suggested file name
    #[prop(into)]
    file_name: Signal<String>,
    /// Signal triggered when the button is clicked
    #[prop(into, optional, default=(signal(ButtonClick::Released).1.into()))]
    click: SignalSetter<ButtonClick>,
    /// Content of the button element
    #[prop(optional, default=Box::new(|| view!{"Press me!"}.into_any()))]
    children: Children,
    /// Style of the button
    #[prop(into, name = "style")]
    Style { kind, schema, size }: Style,
) -> impl IntoView {

    let on_pointerdown = move |event: PointerEvent| {
        let alt = event.alt_key();
        let shift = event.shift_key();

        let is_left = event.button() == 0;
        let is_middle = event.button() == 1;
        let is_right = event.button() == 2;

        if (is_left && alt && shift) || is_middle {
            click.set(ButtonClick::MiddleClick);
        } else if (is_left && alt) || is_right {
            click.set(ButtonClick::RightClick);
        } else if is_left {
            click.set(ButtonClick::LeftClick);
        }

        event.stop_propagation();
        event.cancel_bubble();
        event.prevent_default();
    };

    let on_contextmenu = move |event: MouseEvent| {
        event.stop_propagation();
        event.cancel_bubble();
        event.prevent_default();
    };

    let href = move || {
        let file_content = file_content.get();
        let mime_type = mime_type.get();

        let file_content = general_purpose::STANDARD.encode(file_content);

         format!("data:{mime_type};base64,{file_content}")
    };

    let class = kind.into_button_class(&schema, &size);


    view!{
        <a
            class={class.to_string()}
            href=href
            download=file_name
            on:pointerdown=on_pointerdown
            on:contextmenu=on_contextmenu
            role="button"
        >
            { children() }
        </a>
    }
}


