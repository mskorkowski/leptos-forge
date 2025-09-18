//! Stories related to label

use leptos::prelude::*;
use leptos::web_sys::HtmlElement;

use testing_library_dom::get_by_test_id;
use testing_library_dom::MatcherOptions;
use ui_components::primitives::input::TextInput;
use ui_components::primitives::label::InlineFieldLabel;
use ui_components::primitives::label::TextFieldLabel;
use ui_components::widgets::field::TextField;
use utils_leptos::signal::URwSignal;

use forge::story::Play;
use forge::story::Step;
use forge::Story;

/// Description of the label primitive
const LABEL_DESC: &str = r############"
# Label
 
The `Label` component is used to display text in proximity to the other elements of the ui. For example a label for a text input. In general you should never
reach out for this primitive unless you are implementing your own custom widget.

## Behavior

If text input is empty and there is no placeholder then `Label` should be displayed as a placeholder. 

If user focuses on the empty input the `Label` will be animated to a top left corner of the input.

If user will enter the value then label should stay in the top left corner.

If user removes a value from text input and focuses off the input then label should be animated back from top left corner to the placeholder.

If filed is required then placeholder should be in red color (but only if value was removed).)

> [!IMPORTANT]  
> You must place a `<Label>` primitive after a text input, otherwise it will not transform to a placeholder.

> [!IMPORTANT]  
> You must place a `<Label>` primitive and text input in the `relative` container. Otherwise it will not work as intended

> [!IMPORTANT]
> Your container must provide tailwind `pt-8` (4em?) padding to make the label and input align properly.

**Example:**

```
view!{
    <div class="relative pt-8"> 
        <TextInput id=INPUT_ID text=text />
        <TextFieldLabel for_id=INPUT_ID text=label data_testid="label" />
    </div>
}
```

"############;

/// id of the input field
const INPUT_ID: &str="basic-label-input";

/// story describing the basic label behavior
#[derive(Clone, Copy, Debug)]
pub struct BasicLabelStory {
    /// Signal used to set the value of the label
    label: URwSignal<String>,
    /// Value of text input
    text: URwSignal<String>,
}

impl Default for BasicLabelStory {
    fn default() -> Self {
        let label = URwSignal::new("Basic label".to_string());
        let text = URwSignal::new(String::new());
        BasicLabelStory{
            label,
            text
        }
    }
}

impl Story for BasicLabelStory {
    fn view(&self) -> AnyView {
        let label: Signal<String> = self.label.into();
        let text: URwSignal<String> = self.text;

        view! {
            <div class="relative pt-8"> 
                <TextInput id=INPUT_ID text=text />
                <TextFieldLabel for_id=INPUT_ID text=label data_testid="label" />
            </div>
        }.into_any()
    }

    fn controls(&self) -> AnyView {
        let label: URwSignal<String> = self.label;
        let text: URwSignal<String> = self.text;

        view! {
            <TextField id="leptos-forge-1-label-text-input" text=label label={"Label".to_string()} />
            <TextField id="leptos-forge-1-text-text-input" text=text label={"Text".to_string()} />
        }.into_any()
    }

    fn description(&self) -> &'static str {
        LABEL_DESC
    }

    fn plays(&self) -> Vec<Box<dyn forge::Play<Story=Self>>> {
        vec![
            Box::new(BasicLabelStoryPlayUpdateFromEmpty),
        ]
    }
}

/// Play changes the value of the story signal to check if the value is updated in the UI
struct BasicLabelStoryPlayUpdateFromEmpty;

impl Play for BasicLabelStoryPlayUpdateFromEmpty {
    type Story = BasicLabelStory;

    fn description(&self) -> &'static str {
        "Update the value of the label from the empty to nonempty"
    }

    fn steps(&self) -> Vec<Box<dyn forge::Step<Story = Self::Story>>> {
        vec![
            Box::new(BasicLabelStoryPlayUpdateFromEmptyStepInit),
            Box::new(BasicLabelStoryPlayUpdateFromEmptyStep1),
            Box::new(BasicLabelStoryPlayUpdateFromEmptyStep2),
            Box::new(BasicLabelStoryPlayUpdateFromEmptyStep3),
            Box::new(BasicLabelStoryPlayUpdateFromEmptyStepCleanup),
        ]
    }
}

/// Initializes the label to an empty string
struct BasicLabelStoryPlayUpdateFromEmptyStepInit;

impl Step for BasicLabelStoryPlayUpdateFromEmptyStepInit {
    type Story = BasicLabelStory;

    fn description(&self) -> &'static str {
        "Initialize the label to an empty string"
    }

    fn run(&self, _canvas: &HtmlElement, story: &mut Self::Story) -> Result<(), &'static str> {
        story.label.set(String::default());
        story.text.set(String::default());
        Ok(())
    }
}

/// Validates the state of the label before first step
struct BasicLabelStoryPlayUpdateFromEmptyStep1;

impl Step for BasicLabelStoryPlayUpdateFromEmptyStep1 {
    type Story = BasicLabelStory;
    
    fn description(&self) -> &'static str {
        "Check initial condition of the label"
    }

    fn run(&self, canvas: &HtmlElement, story: &mut Self::Story) -> Result<(), &'static str> {
        if !story.label.get_untracked().is_empty() {
            return Err("Label signal should be empty");
        }

        if !story.text.get_untracked().is_empty() {
            return Err("Text signal should be empty");
        }

        let Ok(label) = get_by_test_id(canvas, "label", MatcherOptions::default()) else {
            return Err("Label should be present");
        };

        let inner_text = label.inner_text();

        if !inner_text.is_empty() {
            return Err("Label should not have any text");
        }
        
        Ok(())
    }
}

/// This step updates the label from empty state to the "New label"
struct BasicLabelStoryPlayUpdateFromEmptyStep2;

impl Step for BasicLabelStoryPlayUpdateFromEmptyStep2 {
    type Story = BasicLabelStory;

    fn description(&self) -> &'static str {
        "Update label from empty state"
    }

    fn run(&self, _canvas: &HtmlElement, story: &mut Self::Story) -> Result<(), &'static str> {
        story.label.set("New label".to_string());

        Ok(())
    }
}

/// Checks if currently label is in state "New label"
struct BasicLabelStoryPlayUpdateFromEmptyStep3;

impl Step for BasicLabelStoryPlayUpdateFromEmptyStep3 {
    type Story = BasicLabelStory;

    fn description(&self) -> &'static str {
        "Check if signal events resulted in the content being updated"
    }

    fn run(&self, canvas: &HtmlElement, story: &mut Self::Story) -> Result<(), &'static str> {
        let Ok(label) = get_by_test_id(canvas, "label", MatcherOptions::default()) else {
            return Err("Label should be present");
        };

        let inner_text = label.inner_text();

        if inner_text.is_empty() {
            return Err("Label should be updated");
        }

        if inner_text != "New label" {
            return Err("Label should have a value `New label`");
        }

        let signal_value = story.label.get_untracked();

        if signal_value != "New label" {
            return Err("Signal should have a value `New label`");
        }

        Ok(())
    }
}

/// Cleanup after the tests are run
struct BasicLabelStoryPlayUpdateFromEmptyStepCleanup;

impl Step for BasicLabelStoryPlayUpdateFromEmptyStepCleanup {
    type Story = BasicLabelStory;

    fn description(&self) -> &'static str {
        "Cleanup after tests"
    }

    fn run(&self, _canvas: &HtmlElement, story: &mut Self::Story) -> Result<(), &'static str> {
        story.label.set("Label".to_string());
        story.text.set("Text".to_string());
        Ok(())
    }
}

/// Description of the label primitive
const INLINE_LABEL_DESC: &str = r############"
# InlineLabel
 
The `InlineLabel` component is used to display text in proximity to the other elements of the ui. For example a label for a checkbox.

## Behavior

For elements like checkboxes, radio buttons and selectors the `Label` should always be displayed to the left or right of the element
(depending on what makes more sense for the input).

"############;

/// id of the input field 1
const INLINE_INPUT_ID_1: &str="inline-label-input-1";
/// id of the input field 2
const INLINE_INPUT_ID_2: &str="inline-label-input-2";
/// id of the input field 3
const INLINE_INPUT_ID_3: &str="inline-label-input-3";
/// if of the input field 4
const INLINE_INPUT_ID_4: &str="inline-level-input-4";

/// story describing the basic label behavior
#[derive(Clone, Copy, Debug)]
pub struct InlineLabelStory {
    /// Signal used to set the value of the label
    label: URwSignal<String>,
}

impl Default for InlineLabelStory {
    fn default() -> Self {
        let label = URwSignal::new("Inline label".to_string());
        InlineLabelStory{
            label,
        }
    }
}

impl Story for InlineLabelStory {
    fn view(&self) -> AnyView {
        let label: Signal<String> = self.label.into();

        view! {
            <div class="relative"> 
                <InlineFieldLabel for_id=INLINE_INPUT_ID_1 text=label/> <input type="text" class="forge-text-standard border-1 border-solid border-gray-800 ml-1" id=INLINE_INPUT_ID_1 /><br/>
                <InlineFieldLabel for_id=INLINE_INPUT_ID_2 text=label/> <input type="checkbox" class="forge-text-standard border-1 border-solid border-gray-800 ml-1" id=INLINE_INPUT_ID_2 /><br/>
                <InlineFieldLabel for_id=INLINE_INPUT_ID_3 text=label/> <input type="radio" class="forge-text-standard border-1 border-solid border-gray-800 ml-1" id=INLINE_INPUT_ID_2 />
                <InlineFieldLabel for_id=INLINE_INPUT_ID_4 text=label/> <TextInput id=INLINE_INPUT_ID_4 text=("sample text".to_string(),) />
            </div>
        }.into_any()
    }

    fn controls(&self) -> AnyView {
        let label: URwSignal<String> = self.label;

        view! {
            <TextField id="leptos-forge-1-label-text-input" text=label label={"Label".to_string()} />
        }.into_any()
    }

    fn description(&self) -> &'static str {
        INLINE_LABEL_DESC
    }
}