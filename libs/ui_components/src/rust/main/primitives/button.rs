//! Button primitive

use leptos::ev::MouseEvent;
use leptos::ev::PointerEvent;
use leptos::prelude::*;

/// Possible button click events
#[non_exhaustive]
#[derive(Default, Clone, Copy, Debug)]
pub enum ButtonClick {
    /// This state can be set to denote that the button is released and no action
    /// should be taken.
    ///
    /// It is useful as the initial stream state.
    #[default]
    Released,
    /// Single click with left mouse button or
    /// keyboard based trigger with <kbd>space</kbd> or <kbd>enter</kbd> key
    LeftClick,
    /// Single click with right mouse button or
    /// <kbd>alt</kbd> with <kbd>space</kbd> or <kbd>enter</kbd> key
    RightClick,
    /// Single click with middle mouse button or
    /// <kbd>alt</kbd> + <kbd>shift</kbd> with <kbd>space</kbd> or <kbd>enter</kbd> key
    MiddleClick,
}

/// Button primitive
///
/// # Usage
///
/// For usability you should always ignore double clicks. For rationale
/// check this article about [double click](https://blog.codinghorror.com/double-click-must-die/)
///
/// # Supported events
///
/// Button supports clicks and double clicks on left, right and middle button using mouse and/or keyboard
///
/// | # | Trigger | Value of `click` signal |
/// |--:|:--------|:------------------------|
/// |  1| Left mouse button click | [`ButtonClick::LeftClick`] |
/// |  2| <kbd>space</kbd> press | [`ButtonClick::LeftClick`] |
/// |  3| <kbd>enter</kbd> press | [`ButtonClick::LeftClick`] |
/// |  4| Right mouse button click | [`ButtonClick::RightClick`] |
/// |  5| <kbd>alt</kbd> + <kbd>space</kbd> press | [`ButtonClick::RightClick`] |
/// |  6| <kbd>alt</kbd> + <kbd>enter</kbd> press | [`ButtonClick::RightClick`] |
/// |  7| <kbd>alt</kbd> + left mouse button | [`ButtonClick::RightClick`] |
/// |  8| Middle mouse button click | [`ButtonClick::MiddleClick`] |
/// |  9| <kbd>alt</kbd> + <kbd>shift</kbd> + <kbd>space</kbd> press | [`ButtonClick::MiddleClick`] |
/// | 10| <kbd>alt</kbd> + <kbd>shift</kbd> + <kbd>enter</kbd> press | [`ButtonClick::MiddleClick`] |
/// | 11| <kbd>alt</kbd> + <kbd>shift</kbd> + left mouse button | [`ButtonClick::MiddleClick`] |
///
/// > *Warning:*
/// >
/// > Some systems (mostly some Linux flavours) use <kbd>alt</kbd> + left mouse button to move windows around
/// > which could prevent the button from being clicked. On other hand if you use Linux your mouse probably has
/// > more then one button.
/// >
///
#[component]
pub fn Button<S1: ToString, S2: ToString>(
    /// Id of the component
    id: S1,
    /// Css classes for the component
    class: S2,
    /// Signal triggered when the button is clicked
    #[prop(into)]
    click: SignalSetter<ButtonClick>,
    /// Content of the button element
    #[prop(optional, default=Box::new(|| view!{"Press me!"}.into_any()))]
    children: Children,
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

    view! {
        <button
            class={class.to_string()}
            id={id.to_string()}
            on:pointerdown=on_pointerdown
            on:contextmenu=on_contextmenu
        >
            {children()}
        </button>
    }
}
