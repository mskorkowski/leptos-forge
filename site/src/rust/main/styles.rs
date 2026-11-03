//! Styles for the components for the leptos-forge site
//! 

use ui_components::model::ButtonColorSchema;

/// Styles for buttons
pub struct Buttons;

impl Buttons {
    pub const BASIC_SCHEMA: ButtonColorSchema = ButtonColorSchema {
        inner: "\
        ",
        outer: "\
        ",
        border: "\
            border-solid \
            border-gray-200 \
            dark:border-gray-600 \
            active:border-forgeblue-500 \
        ",
        color: "\
            text-gray-700 \
            dark:text-gray-200 \
            active:text-gray-800 \
            dark:active:text-gray-100 \
        ",
        background: "\
            bg-gray-200 \
            dark:bg-gray-800 \
            active:bg-forgeblue-300 \
            dark:active:bg-forgeblue-600 \
        "
    };
}

pub struct Styles;

impl Styles {
    pub const BUTTONS: Buttons = Buttons;
}