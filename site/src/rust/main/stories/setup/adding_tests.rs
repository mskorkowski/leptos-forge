//! In this module we keep stories about adding your first tests into `CounterStory` created as part of
//! [setup process][super::SETUP] and it's later refinement [`CounterStory`][super::refine_story::CounterStory] 
//! 

use testing_library_dom::fire_event;
use testing_library_dom::get_by_text;
use testing_library_dom::get_by_test_id;
use testing_library_dom::MatcherOptions;
use testing_library_dom::SelectorMatcherOptions;

use leptos::prelude::*;
use leptos::web_sys::MouseEvent;
use leptos::web_sys::MouseEventInit;

use utils_leptos::signal::URwSignal;
use ui_components::widgets::field::TextField;

use forge::play;
use forge::Play;
use forge::RouteDef;
use forge::Section;
use forge::Story;
use forge::test_id;



/// description of the [Implement the first story][RefineCounterStory] section
const ADDING_TESTS: &str = r#############"
# Adding tests

This chapter continues from the [Implementing first story](setup/first_story) 
and is a third part of our journey to learn the `leptos_forge`.

The last thing we are missing for our `CounterStory` is tests. Like Storybook,
we call them `plays`. To write tests, we will use 
[`testing-library-dom`](https://github.com/RustForWeb/testing-library), 
a rewrite in Rust of the [JS TestingLibrary](https://testing-library.com).

TestingLibrary has excellent [documentation about writing maintainable interaction tests](https://testing-library.com/docs/)
for your components.

`leptos_forge` has a built-in plays runner for your stories. You can run
a test after entering a story. In this section, we will guide you on how
to create your first test and how to run it.

Every `Story` can have one or more `plays`. Every `play` can have multiple
`steps`.

> [!TIP]
>
> In general, your story should have one play only, since there should be 
> only one way to read the story.

> [!NOTE]
>
> If you have an idea about how to improve the testing API, 
> [please add your comment or create an issue](https://github.com/mskorkowski/leptos-forge/issues/6).

## Adding Plays

Our test will check if after crossing the threshold, a message is shown.  

First, let's create a list of steps for our test.  

1. Set the well-known state to the Counter  
2. Check that message is **not** shown yet  
3. Increase the counter  
4. Check that counter was increased and that message **is** shown  
5. Set some sensible values to the Counter  

Now we are going to add a test for our Counter. To do that, we need to 
implement another function from `Story::plays`. This function should return
a list of plays, where each play contains a number of steps.  

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
                    } else {  
                        "".to_string()  
                    }  
                }}  
            </span>  
        </div>  
    }  
}  
```  

Now we are going to add a test for our Counter. To do that, we need to implement
another function from `Story::plays`. This function should return a list of plays,
where each play contains a number of steps.  

```rust  
use leptos::web_sys::{MouseEvent, MouseEventInit};
use testing_library_dom::{
    get_by_text,
    get_by_test_id, 
    fire_event,
    MatcherOptions,
    SelectorMatcherOptions
};
use forge::{Play, play};  

impl Story for TestedCounterStory {  
    ...  

    fn plays(&self) -> Vec<Box<dyn Play<Story=Self>>> {  
        vec![{  
            /// Message to set when running the play  
            const COUNTER_PLAY_MESSAGE: &str = "Value has crossed the threshold";  
            /// Threshold to set when running the play  
            const COUNTER_PLAY_THRESHOLD: i32 = 15_000;  

            play::<Self>("When the counter value crosses the threshold, a message should be shown.")  
                .next(  
                    "Set the well-known state to the Counter",  
                    |_canvas, story| {  
                        story.message.set(COUNTER_PLAY_MESSAGE.to_string());  
                        story.threshold.set(COUNTER_PLAY_THRESHOLD);  
                        story.value.set(COUNTER_PLAY_THRESHOLD);  
                        Ok(())  
                    }  
                )  
                .next(  
                    "Check that message is **not** shown yet",  
                    |canvas, _story| {  
                        let Ok(message_span) = get_by_test_id(canvas, COUNTER_MESSAGE_TEST_ID, MatcherOptions::default()) else {  
                            return Err("Unable to get a message span");  
                        };  

                        let message = message_span.inner_text();  

                        if !message.is_empty() {  
                            return Err("Showing message, while it should be empty");  
                        }  

                        Ok(())  
                    }  
                )  
                .next(  
                    |canvas, _story| { 
                        let Ok(button) = get_by_text(canvas, "+", SelectorMatcherOptions::default()) else {
                            return Err("Can't find the increase button");
                        };
                        
                        let mouse_event_init = MouseEventInit::new();
                        mouse_event_init.set_bubbles(true);
                        mouse_event_init.set_cancelable(true);

                        let Ok(event) = MouseEvent::new_with_mouse_event_init_dict("click", &mouse_event_init) else {
                            return Err("Can't create a click event");
                        };
                        
                        let Ok(_) = fire_event(&button, &event) else {
                            return Err("Event should be fired.");
                        };

                        Ok(()) 
                    }  
                )  
                .next(  
                    "Check that counter was increased and that message **is** shown",  
                    |canvas, _story| {  
                        let Ok(message_span) = get_by_test_id(canvas, COUNTER_MESSAGE_TEST_ID, MatcherOptions::default()) else {  
                            return Err("Unable to get a message span");  
                        };  

                        let message = message_span.inner_text();  

                        if message.is_empty() {  
                            return Err("Message is not visible");  
                        }  

                        if message.as_str() != COUNTER_PLAY_MESSAGE {  
                            return Err("Displaying wrong message");  
                        }  

                        Ok(())  
                    }  
                )  
                .next(  
                    "Set some sensible values to the Counter",  
                    |_canvas, story| {  
                        story.message.set("You work hard!".to_string());  
                        story.threshold.set(10_000);  
                        story.value.set(0);  
                        Ok(())  
                    }  
                )  
                .into()  
        }]  
    }  
}  
``` 

## Running the tests

When you open the `Counter` story in the browser and take a look at the documentation 
panel, the one at the bottom center, then you will find a tab named `tests`.

![Location of the tests tab](/resources/leptos_forge_site/images/guides/adding_tests/01-tests_tab.png)

If you click on it, you should see a view like

![UI for running tests](/resources/leptos_forge_site/images/guides/adding_tests/02-test_runner_ui.png)

On the gray bar you can see a header of a play. Below the play, there is 
a list of steps in a play. Every line follows the same convention:  

1. In a square bracket a status of a play or step  
2. Followed by a name of a play or step.  

At the far right of a play header there are two buttons  

- `Play`: Runs every step of a play  
- `Step`: Runs a single step of a play  

If you press a `Step` button three times, then you should see  

![State of the UI after playing first three steps](/resources/leptos_forge_site/images/guides/adding_tests/03-after_3rd_step.png)  

After you press `Play`, the test will complete and you will see  

![State of the UI after completing the test](/resources/leptos_forge_site/images/guides/adding_tests/04-completed.png)  

"#############;


/// This section describes how you can implement your first story
#[derive(Debug, Default, Clone, Copy)]
pub struct AddingTests;

impl Section for AddingTests {
    fn description(&self) -> &'static str {
        ADDING_TESTS
    }

    fn subroutes(&self) -> Vec<RouteDef> {
        vec![
            RouteDef::story::<TestedCounterStory>("tested_counter_story", "Counter with tests"),
        ]
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
    fn view(&self) -> impl IntoView {
        view!{
            <Counter value={self.value} message={self.message} threshold={self.threshold} />  // <- added missing properties
        }
    }

    fn controls(&self) -> impl IntoView {
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

        view!{
            <TextField id="counter_value" text=value label="Value" default=|| { Some(0.to_string()) } />

            <TextField id="counter_threshold" text=threshold label="Threshold" default=|| { Some(10_000.to_string()) } /> // <- we've added the threshold control field

            <TextField id="counter_message" text={self.message} label="Message" /> // <- we've added a message control field
        }
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
                    |canvas, _story| { 
                        let Ok(button) = get_by_text(canvas, "+", SelectorMatcherOptions::default()) else {
                            return Err("Can't find the increase button");
                        };
                        
                        let mouse_event_init = MouseEventInit::new();
                        mouse_event_init.set_bubbles(true);
                        mouse_event_init.set_cancelable(true);

                        let Ok(event) = MouseEvent::new_with_mouse_event_init_dict("click", &mouse_event_init) else {
                            return Err("Can't create a click event");
                        };
                        
                        let Ok(_) = fire_event(&button, &event) else {
                            return Err("Event should be fired.");
                        };

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
