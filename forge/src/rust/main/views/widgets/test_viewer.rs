//! Widget which shows the tests for the story
//! 

use std::fmt::Display;
use std::time::Duration;

use leptos::attr::Attribute;
use leptos::html::Div;
use leptos::prelude::*;
use reactive_stores::Field;
use reactive_stores::Patch;
use reactive_stores::PatchField;
use reactive_stores::Store;
use reactive_stores::StoreFieldIterator;
use reactive_stores::StorePath;
use ui_components::widgets::details::DetailsParts;

use crate::story::Play;
use crate::story::Step;
use crate::Story;

/// Controls how the execution of the test will be handled
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum ExecutionMode{
    /// Run a single step at a time
    Step,
    /// Play whole test
    Play,
    /// Do not run the test
    #[default]
    Stopped
}

impl PatchField for ExecutionMode {
    fn patch_field(
        &mut self,
        new: Self,
        path: &StorePath,
        notify: &mut dyn FnMut(&StorePath),
    ) {
        if *self != new {
            *self = new;
            notify(path);
        }    
    }
}

/// Result of the test execution
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
enum TestResult {
    /// Success of the test
    Success,
    /// Failure of the test
    Failure(&'static str),
    /// In progress of the test
    InProgress,
    /// Didn't run yet
    #[default]
    NotRun,
}

impl TestResult {
    /// Returns `true` if the test result indicates that test "finished running"
    /// 
    /// As "finished running" we mean that no further steps should be made in the test run.
    /// For example, a test which completed successfully should not run any more steps. On another
    /// case the test which never was run [`NotRun`][TestResult::NotRun] is not considered complete
    /// since potentially all steps need to be run to check if the test passed or failed.
    fn is_complete(&self) -> bool {
        use TestResult::*;
        match self {
            Success | Failure(_) => true,
            InProgress | NotRun => false,
        }
    }
}

impl PatchField for TestResult {
    fn patch_field(
        &mut self,
        new: Self,
        path: &StorePath,
        notify: &mut dyn FnMut(&StorePath),
    ) {
        if *self != new {
            *self = new;
            notify(path);
        }    
    }
}

impl Display for TestResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use TestResult::*;
        match self {
            Success => f.write_str("[SUCCESS]"),
            Failure(reason) => {
                f.write_str("[FAILURE: ")?;
                f.write_str(reason)?;
                f.write_str("]")
            },
            InProgress => f.write_str("[IN PROGRESS]"),
            NotRun => f.write_str("[NOT RUN]"),
        }
    }
}

/// State of the test execution
#[derive(Debug, Clone, Copy, PartialEq, Eq, Store, Patch)]
struct TestState {
    /// id of the test
    /// 
    /// Just the position in the vec, since list of tests is static
    test_id: usize,
    /// Result of the last test execution
    result: TestResult,
}

/// Model describing a state of the test view
#[derive(Debug, Store, Patch)]
struct TestViewModel{
    /// state of the test run
    /// 
    /// This vector always contains the same amount elements as the number of
    /// steps in the [Play] for which the [TestView] was created. There is one
    /// to one correspondence between steps and values in this vector.
    #[store(key: usize = |state| state.test_id)]
    step_results: Vec<TestState>,
    /// next step to be executed
    /// 
    /// Contains the index of the next step in the [step_results][TestViewModel::step_results] and
    /// by the extension index of the step in the [Play::steps()]
    next_step: usize,
    /// execution mode
    /// 
    /// 
    mode: ExecutionMode,
    /// whole test result
    /// 
    /// Result of the whole test
    result: TestResult,
}

impl TestViewModel {
    /// Create new instance of the TestViewModel
    fn new<P: Play>(play: &P) -> Self {

        let steps_count: usize = play.steps().len();
        let mut result: Vec<TestState> = Vec::with_capacity(steps_count);

        for i in 0..steps_count {
            result.push(TestState{
                test_id: i, 
                result: TestResult::NotRun,
            });
        }

        Self{ 
            step_results: result, 
            next_step: 0,
            mode: ExecutionMode::Stopped,
            result: TestResult::NotRun,
        }
    }
}

/// Describes a section of the test viewer 
pub struct TestView<S> 
where
    S: Story,
{
    /// Story which is a source of the test
    story: S,
    /// index of the play with a test to run (inside of the story)
    play: usize,
    /// state of the test
    state: Store<TestViewModel>,
    /// canvas where elements are drawn
    canvas: NodeRef<Div>
}

impl<S> TestView<S> 
where
    S: Story,
{
    /// Create new instance of the TestView
    /// 
    /// # Panics
    /// 
    /// Will panic if the `play` is out of bounds for the `story.plays()`.
    pub fn new(story: S, play: usize, canvas: NodeRef<Div>) -> Self {
        let plays = story.plays();
        let play_to_run = plays.get(play).unwrap();

        let state: Store<TestViewModel> = Store::new(TestViewModel::new(play_to_run));

        Self { 
            story,
            play,
            state,
            canvas,
        }
    }
}

/// Css classes of the buttons used in the test viewer to play tests
fn test_view_button_class<S: ToString>(extra_classes: S) -> impl Attribute {
    let class = format!("storybook-test-view-button bg-forgegray-300 -300 hover:bg-forgeblue-400 active:bg-forgeblue-600 active:text-forgegray-200 px-2 py-2 {}", extra_classes.to_string());

    view!{
        <{..} class=class />
    }
}

/// Runs one step of the tests and updates the UI state accordingly
/// 
/// - `state`: The store containing the test view model
/// - `story`: The mutable reference to the story being tested
/// - `steps`: The list of steps in current play
/// - `canvas`: The reference to the area where user widgets are being drawn
fn run_one_step<S: Story>(
    state: Store<TestViewModel>,
    story: &mut S,
    steps: &[Box<dyn Step<Story=S>>],
    canvas: NodeRef<Div>,
) {
    let next_step: usize = state.next_step().get_untracked();

    if next_step < steps.len() {
        if TestResult::InProgress != state.result().get_untracked() {
            state.result().patch(TestResult::InProgress);
        }

        state.step_results().
            at_unkeyed(next_step).
            result().
            patch(TestResult::InProgress);

        canvas.with(|canvas| {
            if let Some(canvas) = canvas {
                if let Err(e) = steps[next_step].run(canvas, story) {
                    let failure = TestResult::Failure(e);
                    state.step_results().
                        at_unkeyed(next_step).
                        result().
                        patch(failure);
                    state.result().patch(failure);
                } else {
                    state.step_results().
                        at_unkeyed(next_step).
                        result().
                        patch(TestResult::Success);
                    state.next_step().patch(next_step+1);
                };
            } else {
                let failure = TestResult::Failure("Unable to get the reference to the canvas");
                state.step_results().
                        at_unkeyed(next_step).
                        result().
                        patch(failure);
                state.result().patch(failure);
            }
        });
    }
    let next_step: usize = state.next_step().get_untracked();
    if next_step >= steps.len() {
        let last_test_state = state.step_results().
            at_unkeyed(next_step.saturating_sub(1)).
            result().get_untracked();

        state.result().patch(last_test_state);
    }
}

/// Plays the rest of steps in the test. If 
/// 
/// - `delay`: The number of milliseconds to wait between steps
/// - `state`: The store containing the test view model
/// - `story`: The mutable reference to the story being tested
/// - `steps`: The list of steps in current play
/// - `canvas`: The reference to the area where user widgets are being drawn
fn play_steps<S: 'static + Story>(
    delay: u64,
    state: Store<TestViewModel>,
    mut story: S,
    play: usize,
    canvas: NodeRef<Div>,
) {
    let plays: Vec<Box<dyn Play<Story = S> + 'static>> = story.plays();
    let play_to_run: &dyn Play<Story = S> = plays.get(play).unwrap();
    let steps: Vec<Box<dyn Step<Story = S> + 'static>> = play_to_run.steps();
    let next_step: usize = state.next_step().get_untracked();
    let result: TestResult = state.result().get_untracked();

    if next_step < steps.len() &&
       !result.is_complete() {

        run_one_step(state, &mut story, &steps, canvas);

        let result: TestResult = state.result().get_untracked();

        if !result.is_complete() {
            set_timeout(
                move || {
                    play_steps(delay, state, story, play, canvas);
                }, 
                Duration::from_millis(delay)
            );
        }

    }
}



impl<S: 'static + Story> DetailsParts for TestView<S> {
    fn summary(&self) -> leptos::prelude::AnyView {
        let TestView {
            mut story,
            play,
            state,
            canvas
        } = *self;
        
        let plays: Vec<Box<dyn Play<Story = S> + 'static>> = story.plays();
        let play_to_run: &dyn Play<Story = S> = plays.get(play).unwrap();
        let steps: Vec<Box<dyn Step<Story = S> + 'static>> = play_to_run.steps();

        let play_test = move |_| {
            play_steps(500, state, story, play, canvas);
        };
        
        let run_one_step = move |_| {
            run_one_step(state, &mut story, &steps, canvas);
        };

        let result = move || {
            format!("{}", state.result().get())
        };

        (view!{
            <div class="leptos-forge-test-viewer flex flex-row bg-forgegray-100 items-center ">
                <div class="leptos-forge-test-viewer-test-name flex-none text-base font-bold px-2 py-2">{result} - {play_to_run.description()}</div>
                <div class="grow-1" inner_html="&nbsp;"/>
                <div class="leptos-forge-test-viewer-controls flex-none">
                    <button 
                        on:click={play_test}
                        {..test_view_button_class("")}
                    >Play</button>
                    <button
                        on:click={run_one_step}
                        {..test_view_button_class("")}
                    >Step</button>
                </div>
            </div>
        }).into_any()
    }

    fn details(&self) -> leptos::prelude::AnyView {
        let play = self.play;
        let story = self.story;
        let store = self.state;
        let plays = story.plays();
        let play_to_run = plays.get(play).unwrap();
        let steps = play_to_run.steps().
            iter().
            map(|step| step.description()).
            collect::<Vec<_>>();

        (view!{
            <ul class="list-none">
            <For
                each = move || store.step_results()
                key = |result| result.test_id().get()
                let:result
            >
                <StepView 
                    description={
                        let idx = result.test_id().get();
                        if let Some(step) = steps.get(idx) {
                            step
                        }
                        else {
                            "🤯 Unknown test, please report an error"
                        }
                    }
                    state={result}
                />
                </For>
            </ul>
        }).into_any()
    }
}

/// View of the single step
#[component]
fn StepView<S: ToString>(
    /// The name of the step
    description: S,
    /// State of the state view
    #[prop(into)]
    state: Field<TestState>,
) -> impl IntoView {
    
    let test_state = state.result();
    let test_state = move || {
        format!("{}", test_state.get())
    };

    let description = description.to_string();

    view!{
        <li>{test_state} - {description}</li>
    }
}