//! Codearea stories

use leptos::prelude::*;

use ui_components::widgets::field::TextField;
use ui_components::widgets::field::Codearea;
use utils_leptos::signal::URwSignal;

use forge::Story;

/// Description of the empty text field widget story
const CODEAREA_EMPTY_DESC: &str = r############"
# Codearea

This story shows the example of the empty Codearea field

The `Codearea` component allows entering the multiple lines of text by user.

## Behavior of the label

If text input is empty and there is no placeholder then `Label` should be displayed as a placeholder. 

If user focuses on the empty input the `Label` will be animated to a top left corner of the input.

If user will enter the value then label should stay in the top left corner.

If user removes a value from text input and focuses off the input then label should be animated back from top left corner to the placeholder.

If filed is required then placeholder should be in red color (but only if value was removed).)

"############;

/// story describing the basic label behavior
#[derive(Clone, Copy, Debug)]
pub struct BasicCodeareaStory {
    /// Signal used to set the value of the label
    label: URwSignal<String>,
    /// Signal used to set the value of the text field
    text: URwSignal<String>,
}

impl Default for BasicCodeareaStory {
    fn default() -> Self {
        let label: URwSignal<String> = URwSignal::new("Codearea label".to_string());
        let text: URwSignal<String> = URwSignal::new("".to_string());

        BasicCodeareaStory{
            label,
            text,
        }
    }
}

impl Story for BasicCodeareaStory {
    fn view(&self) -> impl IntoView {
        let label: Signal<String> = self.label.into();
        let text: URwSignal<String> = self.text;

        view! {
            <Codearea id="basic-input" text=text label=label />
        }
    }

    fn controls(&self) -> impl IntoView {
        let label: URwSignal<String> = self.label;
        let text: URwSignal<String> = self.text;

        view! {
            <TextField id="leptos-forge-1-label-text-input" text=label label={"Label".to_string()} />
            <TextField id="leptos-forge-1-label-text-input" text=text label={"Text".to_string()} />
        }
    }

    fn description(&self) -> &'static str {
        CODEAREA_EMPTY_DESC
    }
}

/// Description of the empty text field widget story
const CODEAREA_NONEMPTY_DESC: &str = r############"
# Codearea
# Nonempty

This story shows the example of the nonempty text field

The `Codearea` component allows entering the multiple lines of text by user.

## Behavior of the label

If text input is empty and there is no placeholder then `Label` should be displayed as a placeholder. 

If user focuses on the empty input the `Label` will be animated to a top left corner of the input.

If user will enter the value then label should stay in the top left corner.

If user removes a value from text input and focuses off the input then label should be animated back from top left corner to the placeholder.

If filed is required then placeholder should be in red color (but only if value was removed).)

"############;

/// story describing the basic label behavior
#[derive(Clone, Copy, Debug)]
pub struct NonemptyCodeareaStory {
    /// Signal used to set the value of the label
    label: URwSignal<String>,
    /// Signal used to set the value of the text field
    text: URwSignal<String>,
}

impl Default for NonemptyCodeareaStory {
    fn default() -> Self {
        let label: URwSignal<String> = URwSignal::new("Text field label".to_string());
        let text: URwSignal<String> = URwSignal::new("Text field value".to_string());

        NonemptyCodeareaStory{
            label,
            text,
        }
    }
}

impl Story for NonemptyCodeareaStory {
    fn view(&self) -> impl IntoView {
        let label: Signal<String> = self.label.into();
        let text: URwSignal<String> = self.text;

        view! {
            <Codearea id="leptos-forge-1-label-text-input" text=text label=label />
        }
    }

    fn controls(&self) -> impl IntoView {
        let label: URwSignal<String> = self.label;
        let text: URwSignal<String> = self.text;

        view! {
            <TextField id="leptos-forge-1-label-text-input" text=label label={"Label".to_string()} />
            <TextField id="leptos-forge-1-label-text-input" text=text label={"Text".to_string()} />
        }
    }

    fn description(&self) -> &'static str {
        CODEAREA_NONEMPTY_DESC
    }
}