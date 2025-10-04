//! Stories related to the password field

use forge::RouteDef;
use leptos::prelude::*;

use ui_components::model::Password;
use ui_components::primitives::input::button::PasswordButtonStates;
use ui_components::widgets::field::PasswordField;
use ui_components::widgets::field::TextField;
use utils_leptos::signal::URwSignal;

use forge::Story;


/// Description of the empty password field widget story
const PASSWORD_FIELD_EMPTY_DESC: &str = r############"
# PasswordField

This story shows the example of the empty password field

The `PasswordField` component allows entering the single line of text by user.

## Behavior of the label

If text input is empty and there is no placeholder then `Label` should be displayed as a placeholder. 

If user focuses on the empty input the `Label` will be animated to a top left corner of the input.

If user will enter the value then label should stay in the top left corner.

If user removes a value from text input and focuses off the input then label should be animated back from top left corner to the placeholder.

If filed is required then placeholder should be in red color (but only if value was removed).)

"############;

/// story describing the basic password field behavior
#[derive(Clone, Copy, Debug)]
pub struct BasicPasswordFieldStory {
    /// Signal used to set the value of the label
    label: URwSignal<String>,
    /// Signal used to set the value of the text field
    text: URwSignal<Password>,
}

impl Default for BasicPasswordFieldStory {
    fn default() -> Self {
        let label: URwSignal<String> = URwSignal::new("Password".to_string());
        let text: URwSignal<Password> = URwSignal::new(
            Password::new(String::default())
        );

        BasicPasswordFieldStory{
            label,
            text,
        }
    }
}

impl Story for BasicPasswordFieldStory {
    fn view(&self) -> impl IntoView {
        let label: Signal<String> = self.label.into();
        let text: URwSignal<Password> = self.text;

        view! {
            <PasswordField id="basic-input" password=text label=label />
        }
    }

    #[allow(unsafe_code)]
    fn controls(&self) -> impl IntoView {
        let label: URwSignal<String> = self.label;
        let text: URwSignal<String> = self.text.map(
            |password| {
                // Safety: User must be able to see password in the ui
                unsafe {
                    password.get_raw_password_value().to_string()
                }
            },
            |password: &mut Password, new_password: String| {
                // Safety: User must be able to update the password in the ui
                unsafe {
                    password.set_raw_password_value(new_password)
                }
            }
        );

        view! {
            <TextField id="leptos-forge-1-label-text-input" text=label label={"Label".to_string()} />
            <TextField id="leptos-forge-1-label-text-input" text=text label={"Text".to_string()} />
        }
    }

    fn description(&self) -> &'static str {
        PASSWORD_FIELD_EMPTY_DESC
    }

    fn subroutes(&self) -> Vec<RouteDef> {
        vec![
            RouteDef::page::<NonemptyPasswordFieldStory>("nonempty", "Nonempty"),
            RouteDef::page::<NonemptyVisiblePasswordFieldStory>("visible", "Visible"),
        ]
    }
}

/// Description of the nonempty password field widget story
const PASSWORD_FIELD_NONEMPTY_DESC: &str = r############"
# PasswordField
# Nonempty

This story shows the example of the nonempty password field where value is hidden

The `PasswordField` component allows entering the single line of text by user.

## Behavior of the label

If text input is empty and there is no placeholder then `Label` should be displayed as a placeholder. 

If user focuses on the empty input the `Label` will be animated to a top left corner of the input.

If user will enter the value then label should stay in the top left corner.

If user removes a value from text input and focuses off the input then label should be animated back from top left corner to the placeholder.

If filed is required then placeholder should be in red color (but only if value was removed).)

"############;

/// story describing the basic password field behavior when being nonempty
#[derive(Clone, Copy, Debug)]
pub struct NonemptyPasswordFieldStory {
    /// Signal used to set the value of the label
    label: URwSignal<String>,
    /// Signal used to set the value of the text field
    text: URwSignal<Password>,
}

impl Default for NonemptyPasswordFieldStory {
    fn default() -> Self {
        let label: URwSignal<String> = URwSignal::new("Password".to_string());
        let text: URwSignal<Password> = URwSignal::new(
            Password::new("simplePassword123".to_string())
        );

        NonemptyPasswordFieldStory{
            label,
            text,
        }
    }
}

impl Story for NonemptyPasswordFieldStory {
    fn view(&self) -> impl IntoView {
        let label: Signal<String> = self.label.into();
        let text: URwSignal<Password> = self.text;

        view! {
            <PasswordField id="basic-input" password=text label=label />
        }
    }

    #[allow(unsafe_code)]
    fn controls(&self) -> impl IntoView {
        let label: URwSignal<String> = self.label;
        let text: URwSignal<String> = self.text.map(
            |password| {
                // Safety: User must be able to see password in the ui
                unsafe {
                    password.get_raw_password_value().to_string()
                }
            },
            |password: &mut Password, new_password: String| {
                // Safety: User must be able to update the password in the ui
                unsafe {
                    password.set_raw_password_value(new_password)
                }
            }
        );

        view! {
            <TextField id="leptos-forge-1-label-text-input" text=label label={"Label".to_string()} />
            <TextField id="leptos-forge-1-label-text-input" text=text label={"Text".to_string()} />
        }
    }

    fn description(&self) -> &'static str {
        PASSWORD_FIELD_NONEMPTY_DESC
    }
}

/// Description of the nonempty password field widget story when password is set to be visible
const PASSWORD_FIELD_NONEMPTY_VISIBLE_DESC: &str = r############"
# PasswordField
# Nonempty and visible

This story shows the example of the nonempty password field where value is visible

The `PasswordField` component allows entering the single line of text by user.

## Behavior of the label

If text input is empty and there is no placeholder then `Label` should be displayed as a placeholder. 

If user focuses on the empty input the `Label` will be animated to a top left corner of the input.

If user will enter the value then label should stay in the top left corner.

If user removes a value from text input and focuses off the input then label should be animated back from top left corner to the placeholder.

If filed is required then placeholder should be in red color (but only if value was removed).)

"############;

/// story describing the basic password field behavior
#[derive(Clone, Copy, Debug)]
pub struct NonemptyVisiblePasswordFieldStory {
    /// Signal used to set the value of the label
    label: URwSignal<String>,
    /// Signal used to set the value of the text field
    text: URwSignal<Password>,
}

impl Default for NonemptyVisiblePasswordFieldStory {
    fn default() -> Self {
        let label: URwSignal<String> = URwSignal::new("Password".to_string());
        let text: URwSignal<Password> = URwSignal::new(
            Password::new("simplePassword123".to_string())
        );

        NonemptyVisiblePasswordFieldStory{
            label,
            text,
        }
    }
}

impl Story for NonemptyVisiblePasswordFieldStory {
    fn view(&self) -> impl IntoView {
        let label: Signal<String> = self.label.into();
        let text: URwSignal<Password> = self.text;

        view! {
            <PasswordField id="basic-input" password=text label=label state={(PasswordButtonStates::Visible,)}/>
        }
    }

    #[allow(unsafe_code)]
    fn controls(&self) -> impl IntoView {
        let label: URwSignal<String> = self.label;
        let text: URwSignal<String> = self.text.map(
            |password| {
                // Safety: User must be able to see password in the ui
                unsafe {
                    password.get_raw_password_value().to_string()
                }
            },
            |password: &mut Password, new_password: String| {
                // Safety: User must be able to update the password in the ui
                unsafe {
                    password.set_raw_password_value(new_password)
                }
            }
        );

        view! {
            <TextField id="leptos-forge-1-label-text-input" text=label label={"Label".to_string()} />
            <TextField id="leptos-forge-1-label-text-input" text=text label={"Text".to_string()} />
        }
    }

    fn description(&self) -> &'static str {
        PASSWORD_FIELD_NONEMPTY_DESC
    }
}