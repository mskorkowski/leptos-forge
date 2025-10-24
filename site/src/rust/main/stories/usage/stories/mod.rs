//! This module documents how to use `Story`
//!

pub mod testing;

use forge::RouteDef;
use forge::Section;
use testing::TestingSection;

/// Description of the [StorySection]
const STORY: &str = r############"
# Story

Stories are the main way in which you describe your components. As the name 
suggests they are telling a story of intended usage of your component.

Story allows you to

- display your component on `canvas`
- control your component by interacting with it or by changing its state using `control panel`
- show the documentation in the `description` panel
- run tests of your component using `test runner`

## Creating the story

To create the story you must implement the `leptos_forge::Story` trait. All
functions in the `Story` have default implementations. When you create a story
you should override whatever you need to make the story work the best for you.

The simplest, empty, story will look like this

```rust
use leptos_forge::Story;

#[derive(Default, Clone, Copy)]
pub struct MyStory;

impl Story for MyStory {
}
```

We must now register story so it would be possible to navigate. You can do it
by adding the line marked by comment to the [`ROUTES`](/documentation/routes). 

```rust

const ROUTES: &[RouteDef] = &[
    ...
    RouteDef::story::<MyStory>("my_story", "My Story", &[]), // <- Add this line
    ...
];
```

You can read more about [`ROUTES` here](/documentation/routes).

## Story trait

All methods in `Story` have default implementation. In most of the cases you
will be overriding all of the methods.

| Method signature                                    | Description                                                                     | Default behavior|
|:----------------------------------------------------|:--------------------------------------------------------------------------------|:--------------------------------------------------------|
| `fn view(&self) -> impl IntoView`                   | Returns the component related to the story to be added to the canvas.           | Returns empty view                                      |
| `fn controls(&self) -> impl IntoView`               | Returns the control panel component for your story                              | Returns empty view                                      | 
| `fn description(&self) -> &'static str`             | Returns the story you would like to tell about the component                    | Returns a description with how to start writing a story |
| `fn plays(&self) -> Vec<Box<dyn Play<Story=Self>>>` | Returns the list of [tests](/http://localhost:8000/documentation/story/testing) | Returns an empty list of tests                          |

The rest of this document contains the detailed description about creating 
meaningful stories.

> [!NOTE]
>
> #### Why do we need the `Default` trait?
>
> You must implement the `Default` trait so there exists meaningful
> initial state of the story. There is added the benefit of doing it this way
> you pass the story around as a type, which allows stories to be created lazy
> way as you enter their story. 
>
> The consequence of this is the we don't preserve the state of the components 
> between navigation.

## Adding the view and controls

`Story::view` method should return the component you are writing
a story about.

`Story::controls` method should return a view to show the state of the component
the story is talking and allow to change it's state.

```rust
use leptos::prelude::*;
use leptos_forge_ui_components::widgets::field::TextField;

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
            <div>{self.text}</div>
        }
    }

    fn controls(&self) -> impl IntoView {
        view!{
            <TextField id="counter_value" text={self.text} label="Value" />
        }
    }
}
```

Now if you run your `leptos_forge` application then after opening the `My Story`
section you the canvas (at the center) will contain the `Hello, World!` text and 
on the right hand side there will be control panel where you can modify the text
shown.

## Description of the story

A story, as its name implies, should tell the story of how a particular component
can/should be used.

A story should describe 

- the intended usage of the component
  
  Example:

  > This component should be used to present the choice between our products

- when you should use it and when not to use it
  
  Example:

  > Use this button **only** to indicate the default and safe choice for the user

- correspondence with other components and stories
  
  Example:

  > This card is a condensed version of the "critical system error" page.
  > It's important to follow the sections 13.4 - 13.6 of safety guidelines 
  > provided by the customer as the attachment to the specification.

- Benefits and downsides of using a particular component
  
  Example:

  > The table component, if it has too many columns to fit the screen, will cause
  > horizontal overflow. It's bad for users who are not using the mouse as a main
  > navigation tool

For example, the card with a task reminder in CRM system could have a story like

> # Important Task Reminder Card
>
> When a user has an unclosed expired task, we are going to show this task card
> in the top right corner of the work area to remind the user about the most
> important unfinished work. 
> 
> There will be at most 3 task cards shown this way.
> 
> A user can dismiss them by clicking the `[x]` button at the top right corner.
> In such a case, we are going to show the card again after 30 minutes.
> 
> > [!IMPORTANT]
> >
> > You should never use this card for tasks which are
> >
> > - not actionable by the user, for example waiting for customer response
> > - are not expired
> >
> > Our tests show that showing them too much will overwhelm the user. This makes
> > user learn to ignore them and in consequence the whole point of the reminder
> > system will be lost.


This gives the person reading a clear understanding about the context of the
component and the intentions of its usage. This makes it easier for the person
reading to understand when and how to use the component.

### Implementing the description

Currently, we encourage you to create a `const &str` variable in your module and
simply return it from the `description` function.

> [!NOTE]
> We are committed to keep the currently endorsed way working for the foreseeable
> future.
>
> After implementing the [#25 Derive story](https://github.com/mskorkowski/leptos-forge/issues/25)
> what is considered the optimal/encouraged way of writing stories will probably
> change.


Continuing the example of `MyStory`, the example below should explain it the best:

```rust
use leptos_forge::Story;

const MY_STORY: &str = r###########"
# My Story

This component should ...

"###########;

#[derive(Default, Clone, Copy)]
pub struct MyStory;

impl Story for MyStory {
    fn description(&self) -> &'static str {
        MY_STORY
    }
}
```

There are a few reasons for this code layout:
1. In code organization, it looks almost as if `MY_STORY` were a documentation 
  comment for `MyStory`. This gives it a more familiar feel for other programmers.
2. The implementation of the `Story` for `MyStory` is smaller in code size. This
  makes the maintenance of `MyStory` much easier.

## Next steps

To make your stories better you should add [interaction tests](/documentation/story/testing).


"############;

/// Section describing the [forge::Story]
#[derive(Debug, Default, Clone, Copy)]
pub struct StorySection;

impl Section for StorySection {
    fn description(&self) -> &'static str {
        STORY
    }

    fn subroutes(&self) -> Vec<RouteDef> {
        vec![RouteDef::section::<TestingSection>("testing", "Testing")]
    }
}
