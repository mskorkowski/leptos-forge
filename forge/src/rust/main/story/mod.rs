//! Defines story interface and all of fancy stuff that goes with it
//! 

mod tests;

use leptos::prelude::AnyView;
use leptos::prelude::IntoAny;
use leptos::web_sys::HtmlElement;

use leptos::IntoView;
pub use tests::play;
pub use tests::test_id;

use crate::RouteDef;

/// One step in the testing process
/// 
/// Step is an ephemeral description of a single step in the testing process.
/// It will be created and destroyed whenever it's needed, even in the middle of
/// the testing process.
/// 
/// Do not store any data in the structures which implement this trait
pub trait Step {
    /// Story for which the step is defined
    type Story: Story;
    /// Description of the step
    fn description(&self) -> &'static str;
    /// Play the step
    /// 
    /// If the step fails, it should return an error message using
    /// Markdown so it can display it nicely in the UI
    /// 
    /// # Errors
    /// 
    /// If the step fails, it should return an error message which can be displayed in the UI
    /// formatted in Markdown.
    fn run(&self, canvas: &HtmlElement, story: &mut Self::Story) -> Result<(), &'static str>;
}

/// A play for a story
/// 
/// Play is an ephemeral container for a list of steps. It will be freely created and destroyed as needed
/// even in the middle of the testing process. You shouldn't keep any data in the play. 
pub trait Play {
    /// Story related to the play
    type Story: Story;
    /// Description on the play
    fn description(&self) -> &'static str;
    /// List of steps in the play   
    fn steps(&self) -> Vec<Box<dyn Step<Story = Self::Story>>>;
}

impl<T: Play + ?Sized> Play for Box<T> {
    type Story = T::Story;
    fn description(&self) -> &'static str {
        self.as_ref().description()
    }
    fn steps(&self) -> Vec<Box<dyn Step<Story = Self::Story>>> {
        self.as_ref().steps()
    }
}

/// Short tutorial/cheat sheet about [Story] trait implementation
const STORY_DESC: &str = r############"
# New Story
# Cheat sheet

You've just created a new story. There are a few steps to get it working

1. Implement `fn view(&self) -> impl IntoView` so `leptos_forge` can showcase your component
2. Implement `fn controls(&self) -> impl IntoView` so you can control your component and easily test it's behavior in `leptos_forge`
3. Implement `fn description(&self) -> &'static str` where you describe what your component does

## Implementing `fn view(&self) -> impl IntoView`

In here you define how your component should show up in canvas area (grey one at the center) in `leptos_forge`. In most of a cases 
your implementation will be something like this:

```rust
...
    fn view(&self) -> impl IntoView {
        view!{
            <YourComponent prop:my_prop1={self.my_prop1} prop:my_prop2={self.my_prop2} ... />
        }
    }
...
```

## Implementing `fn controls(&self) -> impl IntoView`

In this section you define a set of controls that you can use to change the state of your component. Ready to use components can be
found in the `ui_components::widgets` module.

> [!TIP]
> You can also create your own custom controls that better suit your needs.
>
> This method is here to give you full freedom on how you would like to control your components

## Implementing `fn description(&self) -> &'static str`

It's the last but probably the most important part of implementing the your story.

While creating a description you should try to explain

1. What is your component about
2. How it should/shouldn't be used
3. When you should use it
4. You should describe the properties and their default values

"############;

/// Story to show in the application
/// 
/// # Why `Story` must implement `Copy`?
/// 
/// Story should only by it's nature hold only the data like [Signal][leptos::prelude::Signal] required to manipulate the component via the control panel.
pub trait Story: Default + Copy {
    /// Returns a view of the story
    fn view(&self) -> impl IntoView {}

    /// List of controls for the story
    fn controls(&self) -> impl IntoView {}

    /// Description of the story
    fn description(&self) -> &'static str {
        STORY_DESC
    }

    /// Returns a list of plays for the story
    fn plays(&self) -> Vec<Box<dyn Play<Story=Self>>> {
        Vec::new()
    }

    /// Returns the list of subroutes for the story
    fn subroutes(&self) -> Vec<RouteDef> {
        vec![]
    }
}



/// Type erased [Story]
/// 
/// # Limitations
/// 
/// AnyStory doesn't support running tests
pub struct AnyStory{
    /// returns a view of the story
    view: Box<dyn Fn() -> AnyView>,
    /// returns the controls of the story
    controls: Box<dyn Fn() -> AnyView>,
    /// returns the description of the story
    description: Box<dyn Fn() -> &'static str>,
}

impl AnyStory {
    /// Returns a view of the story
    pub fn view(&self) -> AnyView {
        (self.view)()
    }

    /// List of controls for the story
    pub fn controls(&self) -> AnyView {
        (self.controls)()
    }

    /// Description of the story
    pub fn description(&self) -> &'static str {
        (self.description)()
    }

    /// Create new instance of the `AnyStory`
    pub fn new<S: Story + 'static>(story: S) -> Self {

        let view = move || -> AnyView {
            story.view().into_any()
        };

        let controls = move || -> AnyView {
            story.controls().into_any()
        };

        let description = move || -> &'static str {
            story.description()
        };

        Self {
            view: Box::new(view),
            controls: Box::new(controls),
            description: Box::new(description),
        }
    }
}

impl<S: Story + 'static> From<S> for AnyStory {
    fn from(value: S) -> Self {
        Self::new(value)
    }
}
