//! Module contains the [Page] component

#![allow(clippy::missing_docs_in_private_items)]

use std::marker::PhantomData;

use leptos::html::Div;
use leptos::prelude::*;
use ui_components::widgets::details::Details;
use utils::prelude::ThreadSafe;
use utils_leptos::signal::URwSignal;

use super::canvas::Canvas;
use super::component_panel::ComponentPanel;
use super::control_pane::ControlPane;
use super::description::Description;
use super::tab_panel::TabName;
use crate::IntoStory;
use crate::Story;
use crate::views::canvas::EmbeddedCanvas;
use crate::views::control_pane::EmbeddedControlPane;
use crate::views::tab_panel::Tab;
use crate::views::tab_panel::TabPanel;
use crate::views::widgets::test_viewer::TestView;

/// Page is the view for the single story about a component
#[component]
pub fn Story<S: 'static + IntoStory + Default + Copy + ThreadSafe>(
    /// Phantom data of the story
    #[prop(optional)]
    _story: PhantomData<S>,
) -> impl IntoView {
    let story = S::default().into_story();
    let canvas = NodeRef::new();

    let tabs: Vec<Box<dyn Tab<SidePanelTabs> + 'static>> = vec![
        Box::new(DescriptionTab {
            text: story.description(),
        }),
        Box::new(TestsTabs { story, canvas }),
    ];

    let selector = URwSignal::new(SidePanelTabs::Description);

    view! {
        <>
            <ComponentPanel>
                <Canvas story=story node_ref=canvas />
                <TabPanel
                    id="side-panel"
                    tabs
                    selector
                />
            </ComponentPanel>
            <div class="flex flex-col basis-1/3 first:basis-1/1 px-4 py-4 overflow-auto print:hidden print:basis-0 min-w-xs w-xs shrink-0 @md:shrink-1">
                <ControlPane story=story />
            </div>
        </>
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SidePanelTabs {
    Description,
    Tests,
}

impl TabName for SidePanelTabs {
    fn name(&self) -> &'static str {
        use SidePanelTabs::*;
        match self {
            Description => "Description",
            Tests => "Tests",
        }
    }

    fn html_id(&self) -> &'static str {
        use SidePanelTabs::*;
        match self {
            Description => "description-tab",
            Tests => "tests-tab",
        }
    }

    fn try_from_name(name: &str) -> Option<Self> {
        use SidePanelTabs::*;
        match name {
            "Description" => Some(Description),
            "Tests" => Some(Tests),
            _ => None,
        }
    }
}

struct DescriptionTab {
    text: &'static str,
}

impl Tab<SidePanelTabs> for DescriptionTab {
    fn id(&self) -> SidePanelTabs {
        SidePanelTabs::Description
    }

    fn view(&self) -> AnyView {
        (view! {
            <Description text=self.text />
        })
        .into_any()
    }
}

struct TestsTabs<StoryImpl: Story> {
    story: StoryImpl,
    canvas: NodeRef<Div>,
}

impl<StoryImpl> Tab<SidePanelTabs> for TestsTabs<StoryImpl>
where
    StoryImpl: Story + ThreadSafe,
{
    fn id(&self) -> SidePanelTabs {
        SidePanelTabs::Tests
    }

    fn view(&self) -> AnyView {
        let plays = self.story.plays();
        let canvas = self.canvas;
        let story = self.story;

        if !plays.is_empty() {
            let tests = plays
                .iter()
                .enumerate()
                .map(|(idx, _play)| TestView::new(story, idx, canvas))
                .collect::<Vec<_>>();

            let views = tests
                .iter()
                .map(|test| {
                    view! {
                        <Details details={test} />
                    }
                })
                .collect_view();

            (view! {
                <div class="flex-row basis-1/3 first:basis-1/1 px-4 py-4 overflow-auto">
                    {views}
                </div>
            })
            .into_any()
        } else {
            (view! {
                <div class="flex-row basis-1/3 first:basis-1/1 px-4 py-4 overflow-auto">
                    <div class="leptos-forge-message-box">No tests were defined!</div>
                </div>
            })
            .into_any()
        }
    }
}

#[component]
pub fn EmbeddedStory<S: 'static + IntoStory + Default + Copy + ThreadSafe>(
    /// If set to `true` embedded story will show the canvas with component
    view: bool,
    /// If set to `true` embedded story will show the control panel
    controls: bool,
    /// If set to `true` embedded story will show the description of the story
    ///
    /// Currently we don't support showing description
    #[prop(default = false)]
    #[allow(unused_variables)]
    description: bool,
    /// If set to `true` embedded story will allow you tu run the tests on itself
    ///
    /// Currently we don't support this part
    #[prop(default = false)]
    _tests: bool,
    /// Phantom data of the story
    #[prop(optional)]
    _story: PhantomData<S>,
) -> impl IntoView {
    let canvas_ref = NodeRef::new();
    let story = S::default().into_story();
    let canvas = if view {
        // let v = story.view().into_any();
        Some(view! {
            <EmbeddedCanvas story node_ref=canvas_ref />
        })
    } else {
        None
    };

    let control_pane = if controls {
        Some(view! {
            <EmbeddedControlPane story />
        })
    } else {
        None
    };

    view! {
        <div class="flex flex-col">
            {canvas}
            {control_pane}
        </div>
    }
}
