//! In this module we keep stories about adding your first tests into `CounterStory` created as part of
//! [setup process][super::SETUP] and it's later refinement [`CounterStory`][super::refine_story::CounterStory] 
//! 

use leptos::prelude::*;

use forge::play;
use forge::Play;
use forge::test_id;
use forge::Section;
use forge::Story;

use testing_library_dom::get_by_test_id;
use utils_leptos::signal::URwSignal;
use testing_library_dom::MatcherOptions;
use ui_components::widgets::field::TextField;

/// description of the [Implement the first story][RefineCounterStory] section
const ADDING_TESTS: &str = r#############"
# Adding tests

This chapter continues from the [Implementing first story](setup/first_story) and is a third part of our
journey to learn the `leptos_forge`.

The last thing which we are missing for our `CounterStory` are tests. Like Storybook we call them `plays`.
To write a tests we will use [`testing-library-dom`](https://github.com/RustForWeb/testing-library) a rewrite
to Rust a [JS TestingLibrary](https://testing-library.com).

TestingLibrary has excellent [documentation about writing maintainable tests](https://testing-library.com/docs/) 
for your components. Even if you don't use the `leptos_forge`. 

`leptos_forge` has a built in plays runner for your stories. You can run a test after entering a story. In this 
section we will guide you on how to create your first test and how to run it.

Every `Story` can have one or more `plays`. Every `play` can have multiple `steps`. 

> [!TIP]
>
> In general your story should have one play only, since there should be only one way to read the story.

> [!NOTE]
> 
> If you have an idea about how to improve testing API [please add your comment](https://github.com/mskorkowski/leptos-forge/issues/6)


## Adding plays

Our test will check if after crossing the threshold, a message is shown.

First let's create a list of steps for our test

1. Set the well known state to the Counter
2. Check that message is **not** shown yet
3. Increase the counter
4. Check that counter was increased and that message **is** shown
5. Set some sensible values to the Counter

Now we need to prepare our `Counter` so we can test it. Since we will be checking the presence or lack of message we need to wrap
the area which shows a message in an html `span` element and give it an id. Now our component looks like this

```rust
use forge::test_id; // <- we've added an import

/// Test id of the increase button
const COUNTER_INCREASE_BUTTON_TEST_ID: &str = "counter_increase_button";
/// Test id of the message span
const COUNTER_MESSAGE_TEST_ID: &str = "counter_message";

#[component]
fn Counter(
    /// Value of the counter
    value: URwSignal<i32>,
    /// Message to be shown if threshold is exceeded
    #[prop(into)]
    message: Signal<String>,
    /// Threshold for displaying the message
    #[prop(into)]
    threshold: Signal<i32>,
) -> impl IntoView {
    let button_style = || view!{
        <{..} style="border: 1px solid black; padding: 1em 2em; border-radius: 3px; margin: 0px 2em; cursor: pointer;" />
    };

    view!{
        <div>
            <button 
                on:click=move |_| { 
                    value.set(
                        value.get_untracked() - 1
                    );
                }
                {..button_style()}
            >-</button>
            <span>{value}</span>
            <button 
                on:click=move |_| { 
                    value.set(
                        value.get_untracked() + 1
                    );
                }
                {..button_style()}
                {..test_id(Some(COUNTER_INCREASE_BUTTON_TEST_ID))}  // <- we've added a test id here
            >+</button><br/>
            <span {..test_id(Some(COUNTER_MESSAGE_TEST_ID))}>       // <- we've wrapped message in span tag with test id
                {move || {
                    if value.get() > threshold.get() {
                        message.get()
                    }
                    else {
                        "".to_string()
                    }
                }}
            </span>
        </div>
    }
}

```

Now we are going to add a test for our Counter. To do that we need to implement another function from `Story::plays`.
This function should return a list of plays, where each play contains a number of steps. 



```rust
use forge::play;
use forge::Play;

impl Story for TestedCounterStory {
    ...

    fn plays(&self) -> Vec<Box<dyn Play<Story=Self>>> {
        vec![{
            /// Message to set when running the play
            const COUNTER_PLAY_MESSAGE: &str = "Value has crossed the threshold";
            /// Threshold to set when running the play
            const COUNTER_PLAY_THRESHOLD: i32 = 15_000;

            play::<Self>("When the counter value crosses the threshold, a message should be shown.").
                next(
                    "Set the well known state to the Counter", 
                    |_canvas, story| { 
                        story.message.set(COUNTER_PLAY_MESSAGE.to_string());
                        story.threshold.set(COUNTER_PLAY_THRESHOLD);
                        story.value.set(COUNTER_PLAY_THRESHOLD - 1);
                        Ok(()) 
                    }
                ).
                next(
                    "Check that message is **not** shown yet",
                    |canvas, _story| { 
                        let Ok(message_span) = get_by_test_id(canvas, COUNTER_MESSAGE_TEST_ID, MatcherOptions::default()) else {
                            return Err("Unable to get a message span");
                        };

                        let message = message_span.inner_text();

                        if !message.is_empty() {
                            return Err("Showing message, while it should be empty")
                        }

                        Ok(()) 
                    }
                ).
                next(
                    "Increase the counter",
                    |_canvas, story| { 
                        story.value.set(COUNTER_PLAY_THRESHOLD);
                        Ok(()) 
                    }
                ).
                next(
                    "Check that counter was increased and that message **is** shown",
                    |canvas, _story| { 
                        let Ok(message_span) = get_by_test_id(canvas, COUNTER_MESSAGE_TEST_ID, MatcherOptions::default()) else {
                            return Err("Unable to get a message span");
                        };

                        let message = message_span.inner_text();

                        if message.is_empty() {
                            return Err("Message is not visible")
                        }

                        if message.as_str() != COUNTER_PLAY_MESSAGE {
                            return Err("Displaying wrong message")
                        }

                        Ok(())
                    }
                ).
                next(
                    "Set some sensible values to the Counter",
                    |_canvas, story| { 
                        story.message.set("You work hard!".to_string());
                        story.threshold.set(10_000);
                        story.value.set(0);
                        Ok(()) 
                    }
                ).
                into()
        }]
    }
}

```

"#############;


/// This section describes how you can implement your first story
#[derive(Debug, Default, Clone, Copy)]
pub struct AddingTests;

impl Section for AddingTests {
    fn description(&self) -> &'static str {
        ADDING_TESTS
    }
}

//----------------------------------------------------------------------------------------------------------------
//
// Below is implementation of the Counter component and CounterStory described in the `RefineCounterStory` section
//
//----------------------------------------------------------------------------------------------------------------

/// Test id of the increase button
const COUNTER_INCREASE_BUTTON_TEST_ID: &str = "counter_increase_button";
/// Test id of the message span
const COUNTER_MESSAGE_TEST_ID: &str = "counter_message";

/// Counter component described in [RefineCounterStory] section of the site
#[component]
fn Counter(
    /// Value of the counter
    value: URwSignal<i32>,
    /// Message to be shown if threshold is exceeded
    #[prop(into)]
    message: Signal<String>,
    /// Threshold for displaying the message
    #[prop(into)]
    threshold: Signal<i32>,
) -> impl IntoView {
    let button_style = || view!{
        <{..} style="border: 1px solid black; padding: 1em 2em; border-radius: 3px; margin: 0px 2em; cursor: pointer;" />
    };

    view!{
        <div>
            <button 
                on:click=move |_| { 
                    value.set(
                        value.get_untracked() - 1
                    );
                }
                {..button_style()}
            >-</button>
            <span>{value}</span>
            <button 
                on:click=move |_| { 
                    value.set(
                        value.get_untracked() + 1
                    );
                }
                {..button_style()}
                {..test_id(Some(COUNTER_INCREASE_BUTTON_TEST_ID))}  // <- we've added a test id here
            >+</button><br/>
            <span {..test_id(Some(COUNTER_MESSAGE_TEST_ID))}>       // <- we've wrapped message in span tag with test id
                {move || {
                    if value.get() > threshold.get() {
                        message.get()
                    }
                    else {
                        "".to_string()
                    }
                }}
            </span>
        </div>
    }
}

const COUNTER_STORY: &str = r############"
# Counter

Counter component which allows you to increment or decrement a value.

- When you press `[-]` button, the value will decrease by 1.
- When you press `[+]` button, the value will increase by 1.

If you exceed the threshold (by default 10000) the message will be shown.
The default message is "You are working hard".

"############;

/// Counter story created as part of the [RefineCounterStory] section of the site
/// 
/// This way user can compare the result he got and the one he is expected to get
/// alongside doing a tutorial
#[derive(Debug, Clone, Copy)]
pub struct TestedCounterStory {
    /// Value of the counter
    value: URwSignal<i32>,
    /// Message to be shown if threshold is exceeded
    message: URwSignal<String>,
    /// Threshold for displaying the message
    threshold: URwSignal<i32>,
}

impl Default for TestedCounterStory {
    fn default() -> Self {
        TestedCounterStory {
            value: 0.into(),
            message: "You are working hard".to_string().into(),
            threshold: 10_000.into(),
        }
    }
}

impl Story for TestedCounterStory {
    fn view(&self) -> AnyView {
        (view!{
            <Counter value={self.value} message={self.message} threshold={self.threshold} />  // <- added missing properties
        }).into_any()
    }

    fn controls(&self) -> AnyView {
        let value = self.value.map(
            |v| v.to_string(),
            |v, text| {
                if let Ok(new_value) = text.parse::<i32>() {
                    *v = new_value;
                }
            }, 
        );

        let threshold = self.threshold.map( // <- we've added a `threshold` variable. The transformation is exactly the same as for value
            |v| v.to_string(),
            |v, text| {
                if let Ok(new_value) = text.parse::<i32>() {
                    *v = new_value;
                }
            },
        );

        (view!{
            <TextField id="counter_value" text=value label="Value" default=|| { Some(0.to_string()) } />

            <TextField id="counter_threshold" text=threshold label="Threshold" default=|| { Some(10_000.to_string()) } /> // <- we've added the threshold control field

            <TextField id="counter_message" text={self.message} label="Message" /> // <- we've added a message control field
        }).into_any()
    }

    fn description(&self) -> &'static str {
        COUNTER_STORY
    }

    fn plays(&self) -> Vec<Box<dyn Play<Story=Self>>> {
        vec![{
            /// Message to set when running the play
            const COUNTER_PLAY_MESSAGE: &str = "Value has crossed the threshold";
            /// Threshold to set when running the play
            const COUNTER_PLAY_THRESHOLD: i32 = 15_000;

            play::<Self>("When the counter value crosses the threshold, a message should be shown.").
                next(
                    "Set the well known state to the Counter", 
                    |_canvas, story| { 
                        story.message.set(COUNTER_PLAY_MESSAGE.to_string());
                        story.threshold.set(COUNTER_PLAY_THRESHOLD);
                        story.value.set(COUNTER_PLAY_THRESHOLD);
                        Ok(()) 
                    }
                ).
                next(
                    "Check that message is **not** shown yet",
                    |canvas, _story| { 
                        let Ok(message_span) = get_by_test_id(canvas, COUNTER_MESSAGE_TEST_ID, MatcherOptions::default()) else {
                            return Err("Unable to get a message span");
                        };

                        let message = message_span.inner_text();

                        if !message.is_empty() {
                            return Err("Showing message, while it should be empty")
                        }

                        Ok(()) 
                    }
                ).
                next(
                    "Increase the counter",
                    |_canvas, story| { 
                        story.value.set(COUNTER_PLAY_THRESHOLD+1);
                        Ok(()) 
                    }
                ).
                next(
                    "Check that counter was increased and that message **is** shown",
                    |canvas, _story| { 
                        let Ok(message_span) = get_by_test_id(canvas, COUNTER_MESSAGE_TEST_ID, MatcherOptions::default()) else {
                            return Err("Unable to get a message span");
                        };

                        let message = message_span.inner_text();

                        if message.is_empty() {
                            return Err("Message is not visible")
                        }

                        if message.as_str() != COUNTER_PLAY_MESSAGE {
                            return Err("Displaying wrong message")
                        }

                        Ok(())
                    }
                ).
                next(
                    "Set some sensible values to the Counter",
                    |_canvas, story| { 
                        story.message.set("You work hard!".to_string());
                        story.threshold.set(10_000);
                        story.value.set(0);
                        Ok(()) 
                    }
                ).
                into()
        }]
    }
}
