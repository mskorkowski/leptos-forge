//! This section describes how to wite interaction tests using [Play][forge::Play]
//! and [Step][forge::Step]

use forge::Section;

/// Description of the [PlaysSection]
const PLAY: &str = r############"
# Testing

Tests are called plays in the `leptos_forge` because the should be akin to
the theatrical play based on story about the components.

> [!TIP]
>
> Rule of thumb is that single story should have a single play since there 
> should be only one way to tell the story.
>
> There is one exception from this rule, the same story is activated differently.
> For example you test if "clicking the button with a mouse" will have the same
> effect as "firing the button with a keyboard".

To add plays for your story you should implement the 

| Method signature                                    | Description               | Default behavior|
|:----------------------------------------------------|:--------------------------|:--------------------------------|
| `fn plays(&self) -> Vec<Box<dyn Play<Story=Self>>>` | Returns the list of plays | Returns an empty list of tests  |

The minimal example

```rust
use leptos::prelude::*;
use leptos_forge_ui_components::widgets::field::TextField;
use leptos_forge::{Play, play, test_id };
use testing_library_dom::{get_by_test_id, MatcherOptions};

#[derive(Debug, Clone, Copy)]
pub struct MyStory{
  text: Signal<String>,
}

impl Default {
    fn default() -> Self {
        let (text, _) = signal("Hello, World!".to_string());

        Self {
            text,
        }
    }
}

impl Story for MyStory {

    fn view(&self) -> impl IntoView {
        view!{
            <div 
                {..test_id(Some("my-story-text"))}
            >{self.text}</div>
        }
    }

    fn controls(&self) -> impl IntoView {
        view!{
            <TextField id="counter_value" text={self.text} label="Value" />
        }
    }

    fn plays(&self) -> Vec<Box<dyn Play<Story=Self>>> {
        vec![{
            /// Message we set the content of the text to
            const MESSAGE = "1., 2., 3., Testing...";

            play::<Self>("Check if the text can be updated").
                next(
                    "Updata the text",
                    |_canvas, story| {
                        story.text.set(MESSAGE.to_string());
                    }
                ).
                next(
                    "Check if text was updated",
                    |canvas, _story| {
                        let Ok(div) = get_by_test_id(canvas, "my-story-text", MatcherOptions::default()) {
                            return Err("Div with message was not found on canvas?");
                        }

                        let message = div.inner_text();
                        if message.is_empty() {
                            return Err("Message is not visible");
                        }

                        if message_as_str() != MESSAGE {
                            return Err("Wrong message is shown!")
                        }

                        OK(())
                    }
                )
        }]
    }
}
```

In the example above we've created a simple div which just prints the value held
by the signal (`MyStory::view method`). We've also added a simple control panel 
with a single input field which allows updating the displayed text 
(`MyStory::controls` method).

The interesting part start with the implementation of `MyStory::plays` method.

To create a play we use function `leptos_forge::play`. 

"############;

/// Section about [Play][forge::Play]
#[derive(Debug, Default, Clone, Copy)]
pub struct TestingSection;

impl Section for TestingSection {
    fn description(&self) -> &'static str {
        PLAY
    }
}
