//! Provides a builder for creating tests in the [stories][Story::]
//! 

use super::Play;
use super::Step;
use super::Story;

use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::IntoAnyAttribute;
use leptos::view;
use leptos::web_sys::HtmlElement;

/// Type of the function which is used for steps in the [Play]
type StepFn<S> = fn(canvas: &HtmlElement, &mut S) -> Result<(), &'static str>;

/// Simple implementation of the step interface should be enough for most of the use cases
#[derive(Clone)]
pub struct SimpleStep<S: Story> {
    /// Description of the step shown in the UI
    /// 
    /// It should be short, one liner if possible, which will describe the actions taken
    /// during this step.
    /// 
    /// It will be shown in the test runner UI.
    description: &'static str,
    /// Function to run when the step is played
    step: StepFn<S>,
}

impl<S: Story> SimpleStep<S> {
    /// Create new instance of the SimpleStep
    pub fn new(description: &'static str, step: StepFn<S>) -> Self {
        Self { description, step }
    }
}

impl<S: Story> Step for SimpleStep<S> {
    type Story = S;

    fn description(&self) -> &'static str {
        self.description
    }

    fn run(&self, canvas: &HtmlElement, story: &mut Self::Story) -> Result<(), &'static str> {
        (self.step)(canvas, story)
    }
}

impl<S: Story + 'static> From<SimpleStep<S>> for Box<dyn Step<Story = S>> {
    fn from(val: SimpleStep<S>) -> Self {
        Box::new(val)
    }
}

/// The simple implementation of the play interface should be enough for most of the use cases
/// 
/// It doubles as the factory for creating a tests
pub struct SimplePlay<S: Story> {
    /// Description of the play
    /// 
    /// It should be short, one liner if possible, which will describe what this play
    /// is about in relation to the story.
    /// 
    /// It will be shown in the UI as the header above the tests
    description: &'static str,

    /// list of steps to run a test
    steps: Vec<SimpleStep<S>>,
}

impl<S: Story> SimplePlay<S> {
    /// adds next step to the play
    pub fn next(mut self, name: &'static str, step: StepFn<S>) -> Self {
        self.steps.push(SimpleStep::new(name, step));
        self
    }
}

impl<S: Story + 'static> From<SimplePlay<S>> for Box<dyn Play<Story = S>> {
    fn from(value: SimplePlay<S>) -> Self {
        Box::new(value)
    }
}

impl<S: Story + 'static> Play for SimplePlay<S> {
    type Story = S;

    fn description(&self) -> &'static str {
        self.description
    }

    fn steps(&self) -> Vec<Box<dyn Step<Story = Self::Story>>> {
        self.steps.
            iter().
            map(|step| step.clone().into()).
            collect::<Vec<_>>()
    }
}


/// Create a play for the story
pub fn play<S: Story>(name: &'static str) -> SimplePlay<S> {
    SimplePlay{
        description: name,
        steps: Vec::new(),
    }
}

/// Add test id to the element
pub fn test_id<S: ToString>(test_id: Option<S>) -> AnyAttribute {
    if let Some(test_id) = test_id {
        (view!{
            <{..} data-testid={test_id.to_string()} />
        }).into_any_attr()
    }
    else {
        ().into_any_attr()
    }
}