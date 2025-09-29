//! This module contains a section where we will refine the `ButtonStory` created in the setup part

use leptos::prelude::*;

use ui_components::widgets::field::TextField;
use utils_leptos::signal::URwSignal;

use forge::Section;
use forge::Story;

/// description of the [Implement the first story][RefineCounterStory] section
const REFINE_COUNTER_STORY: &str = r#############"
# Implementing the first story

This section builds upon the code which was created during the [create project guide](/guides) part. 
The expected result of this section can be checked by going to the 
[`Counter story`](/guides/first_story/counter_story) subpage.

## First things first, we need a component

We will start with creating a simple component. This component will have a single 
button. Every time we click `+`, it will increase a value of the counter by one.
 Every time we click `-`, it will decrease a value of the counter by one.

In the `src/stories.rs` file we will add the following leptos component:

```rust
use leptos::prelude::*;
use utils_leptos::signal::URwSignal;

/// Counter component described in [RefineCounterStory] section of the site
#[component]
fn Counter(
    /// Value of the counter
    value: URwSignal<i32>
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
            >+</button>
        </div>
    }
}
```

This component is a simple counter with two buttons. One to increase and the 
other to decrease the counter's value. Only non-standard feature is using the 
`URwSignal` type from the `utils_leptos` crate. `URwSignal` is a generic 
read-write signal with a little bit of extra sauce.

## Updating the story

Our `Counter` component takes a single `value: URwSignal<i32>` as a property. 
We need to add this value to our story. To do that, we need to change the 
definition of the `CounterStory` struct to include a `value` field.

```rust
#[derive(Debug, Default, Clone, Copy)]
pub struct CounterStory {
    /// Value of the counter
    value: URwSignal<i32>,
}
```

Now we can show the component in our story. To do that, we add an implementation
of the `view` function to the implementation of the `Story` trait for `CounterStory`.

```rust
impl Story for CounterStory {
    fn view(&self) -> AnyView {
        (view!{
            <Counter value={self.value} />
        }).into_any()
    }
}
```

Whatever you return **from** `Story::view` function will be shown in the canvas 
area.

Now when you open the `Button` story at the top left side of the gray area you
will find the component. It should look like this:

![Example counter component rendered in the canvas area](/resources/leptos_forge_site/images/guides/refine_story/01-component_view.png)

You can test our component by clicking on the `[-]` and `[+]` buttons.

Now our simple story needs a way to control our component, so we can set a given
state for our counter. This makes **it** easier to set a desired state. To do that,
we need to implement another function in our implementation of the `Story` for
`CounterStory`.

```rs
use ui_components::widgets::field::TextField; // <- We need to add this use statement
so we can use TextField widget

impl Story for CounterStory {
    fn view(&self) -> AnyView {
       ...
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

        (view!{
            <TextField id="counter_value" text=value label="Value" default=|| { Some(0.to_string()) } />
        }).into_any()
    }
}
```

The `Story::controls` function allows you to implement the control panel. Whatever 
you return from this function will be rendered as a control panel.

> [!NOTE]
> 
> By design `leptos_forge` is giving you a full control over what is rendered as 
> a control panel. This is one of the key features of `leptos_forge`.

Let's deep dive into the code of the function we just created.

**First, we create** a new variable `value`. This variable holds the current value 
of an input field we will use to update our state. To do that we use 
a `URwSignal::map` function. This function takes two arguments:

1. A function which takes the current value of the signal and returns a new value
   based on it.
2. A function which takes a mutable reference `v` to the current value and a new value
   to which the `value` signal was set and updates (or not) the `v`.

The `text` variable has `URwSignal<String>` type.

Afterwards we use leptos `view!` to create our control panel. To do that we use a 
`TextField` component provided by the `leptos_forge_ui_components` crate. The 
arguments passed to this component are:

- `id`: The id of the input field
- `text`: The `URwSignal<String>` which holds the value of the input field.
- `label`: The `str` which will be shown as a label
- `default`: The function which returns a `String`. This function will be 
   called if you click on the `clear` input button.

Now if you go to the `Counter` story in your web browser, you will see:

![Counter component and its control panel](/resources/leptos_forge_site/images/guides/refine_story/02-counter_with_control.png)

After three clicks on the `[+]` button you will see:

![Counter component and its control panel after third click on the plus button](/resources/leptos_forge_site/images/guides/refine_story/03-counter_with_control-3rd_click.png)

If you would like to change the current value of the counter to `3000`, you can 
just go to the control panel and add a few zeros **to** the input field.

And you end up with:

![Counter component and its control panel after writing 3000 in the input field](/resources/leptos_forge_site/images/guides/refine_story/04-counter_with_control-3000.png)

## Let's spice it up

Until now our component was fairly simple and no huge gains were made by using
the `leptos_forge`. Let's add a requirement for our counter. If the counter has
a value greater than 10000, then we should display a message "You are working hard".

Our new counter can be written as follows

```rust
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
            >+</button><br/>
            {move || {
                if value.get() > threshold.get() {
                    message.get()
                }
                else {
                    "".to_string()
                }
            }}
        </div>
    }
}
```

Now we need to update our `CounterStory` struct so we can provide the required
signals to our `Counter` component.

```rust
#[derive(Debug, Clone, Copy)]
pub struct CounterStory {
    /// Value of the counter
    value: URwSignal<i32>,
    /// Message to be shown if threshold is exceeded
    message: URwSignal<String>,
    /// Threshold for displaying the message
    threshold: URwSignal<i32>,
}

impl Default for CounterStory {
    fn default() -> Self {
        CounterStory {
            value: 0.into(),
            message: "You are working hard".to_string().into(),
            threshold: 10_000.into(),
        }
    }
}
```

Because the default state of our `Counter` component is no longer what Rust
considers a default, we had to implement the `Default` trait manually.

Now we will update the implementation of the `Story` for `CounterStory`.

```rust
impl Story for CounterStory {
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

        let threshold = self.threshold.map( // <- we've added a `threshold` variable. The transformation is exactly the same as for the value
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
}
```

Now if you need to check how the `Counter` component will behave in various
scenarios, you can just go to the browser and adjust values within the control
panel we've created. It's faster than clicking `[+]` button 10000 times.

## Documentation

The next step we can take is to write documentation for our `Counter` which
will be shown on the right-hand side whenever you open the `Counter` story.

To write documentation, we need to implement `Story::description` function.
This function must return a `&'static str` which contains a markdown-formatted
text.

I find it easiest to write documentation by creating a constant value and then
using it within the `description` function.

```rust
const COUNTER_STORY: &str = r############"
# Counter

Counter component which allows you to increment or decrement a value.

- When you press `[-]` button, the value will decrease by 1.
- When you press `[+]` button, the value will increase by 1.

If you exceed the threshold (by default 10000), the message will be shown.
The default message is "You are working hard".

"############;

impl Story for CounterStory {
    ...

    fn description(&self) -> &'static str {
        COUNTER_STORY
    }
}
```

Now on the bottom you should see

![Documentation of the counter](/resources/leptos_forge_site/images/guides/refine_story/05-counter_documentation.png)

## Next steps

We've just implemented a basic story for our counter component. The next step is to [add tests](setup/adding_tests).

"#############;


/// This section describes how you can implement your first story
#[derive(Debug, Default, Clone, Copy)]
pub struct RefineCounterStory;

impl Section for RefineCounterStory {
    fn description(&self) -> &'static str {
        REFINE_COUNTER_STORY
    }
}

//----------------------------------------------------------------------------------------------------------------
//
// Below is implementation of the Counter component and CounterStory described in the `RefineCounterStory` section
//
//----------------------------------------------------------------------------------------------------------------



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
            >+</button><br/>
            {move || {
                if value.get() > threshold.get() {
                    message.get()
                }
                else {
                    "".to_string()
                }
            }}
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
pub struct CounterStory {
    /// Value of the counter
    value: URwSignal<i32>,
    /// Message to be shown if threshold is exceeded
    message: URwSignal<String>,
    /// Threshold for displaying the message
    threshold: URwSignal<i32>,
}

impl Default for CounterStory {
    fn default() -> Self {
        CounterStory {
            value: 0.into(),
            message: "You are working hard".to_string().into(),
            threshold: 10_000.into(),
        }
    }
}

impl Story for CounterStory {
    fn view(&self) -> impl IntoView {
        (view!{
            <Counter value={self.value} message={self.message} threshold={self.threshold} />  // <- added missing properties
        }).into_any()
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

        (view!{
            <TextField id="counter_value" text=value label="Value" default=|| { Some(0.to_string()) } />

            <TextField id="counter_threshold" text=threshold label="Threshold" default=|| { Some(10_000.to_string()) } /> // <- we've added the threshold control field

            <TextField id="counter_message" text={self.message} label="Message" /> // <- we've added a message control field
        }).into_any()
    }

    fn description(&self) -> &'static str {
        COUNTER_STORY
    }
}