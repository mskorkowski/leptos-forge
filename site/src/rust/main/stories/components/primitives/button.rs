//! Stories for Button primitive
//! 

use leptos::prelude::*;
use ui_components::primitives::button::Button;
use ui_components::primitives::button::ButtonClick;
use ui_components::widgets::field::TextField;
use ui_components::widgets::field::ReadonlyField;
use utils_leptos::signal::URwSignal;

use forge::Story;

/// Description of the switch primitive when toggled of
const BUTTON_DESC: &str = r############"
# Button

Button is a component that allows users to click on it so they can trigger an action

## Usage
 
For usability you should always ignore double clicks. For rationale
check this article about [double click](https://blog.codinghorror.com/double-click-must-die/)
 
## Supported events
 
Button supports clicks and double clicks on left, right and middle button using mouse and/or keyboard
 
| # | Trigger | Value of `click` signal |
|--:|:--------|:------------------------|
|  1| Left mouse button click | [`ButtonClick::LeftClick`] |
|  2| <kbd>space</kbd> press | [`ButtonClick::LeftClick`] |
|  3| <kbd>enter</kbd> press | [`ButtonClick::LeftClick`] |
|  4| Right mouse button click | [`ButtonClick::RightClick`] |
|  5| <kbd>alt</kbd> + <kbd>space</kbd> press | [`ButtonClick::RightClick`] |
|  6| <kbd>alt</kbd> + <kbd>enter</kbd> press | [`ButtonClick::RightClick`] |
|  7| <kbd>alt</kbd> + left mouse button | [`ButtonClick::RightClick`] |
|  8| Middle mouse button click | [`ButtonClick::MiddleClick`] |
|  9| <kbd>alt</kbd> + <kbd>shift</kbd> + <kbd>space</kbd> press | [`ButtonClick::MiddleClick`] |
| 10| <kbd>alt</kbd> + <kbd>shift</kbd> + <kbd>enter</kbd> press | [`ButtonClick::MiddleClick`] |
| 11| <kbd>alt</kbd> + <kbd>shift</kbd> + left mouse button | [`ButtonClick::MiddleClick`] |
 
> [!Warning]
> Some systems (mostly some Linux flavours) use <kbd>alt</kbd> + left mouse button to move windows around
> which could prevent the button from being clicked. On other hand if you use Linux your mouse probably has
> more then one button.

"############;

/// Basic switch story
#[derive(Clone, Copy)]
pub struct BasicButtonStory {
    /// Signal which triggers on clicks
    click: URwSignal<ButtonClick>,
    /// Text shown inside a button
    text: URwSignal<String>,
}

impl Default for BasicButtonStory {
    fn default() -> Self {
        Self{ 
            click: URwSignal::new(ButtonClick::Released),
            text: URwSignal::new("Button".to_string())
        }
    }
}

impl Story for BasicButtonStory {
    fn description(&self) -> &'static str {
        BUTTON_DESC
    }

    fn controls(&self) -> impl IntoView {
        let click = self.click;
        let state = Signal::derive(move || {
            let state = click.get();
            format!("{state:?}")
        });

        view!{
            <>
                <TextField id="button-text" text={self.text} label="Button label" />
                <ReadonlyField id="button-state" value={state} label="State" />
            </>
        }.into_any()
    }

    fn view(&self) -> impl IntoView {
        let text: URwSignal<String> = self.text;

        view!{
            <Button
                id="button_id"
                class="bg-forgegray-100 border-forgegray-400 h-12 inline-block px-4 hover:bg-forgegray-200 active:bg-forgegray-300"
                click={self.click}
            >
               {text}
            </Button>
        }.into_any()
    }
}