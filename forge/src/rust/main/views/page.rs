//! Module contains the [Page] component

#![allow(clippy::missing_docs_in_private_items)]


use std::marker::PhantomData;

use leptos::html::Div;
use leptos::prelude::*;
use ui_components::widgets::details::Details;
use utils_leptos::signal::ThreadSafe;
use utils_leptos::signal::URwSignal;


use crate::views::tab_panel::Tab;
use crate::views::tab_panel::TabPanel;
use crate::views::widgets::test_viewer::TestView;
use crate::Story;
use super::canvas::Canvas;
use super::control_pane::ControlPane;
use super::component_panel::ComponentPanel;
use super::description::Description;
use super::tab_panel::TabName;


/// Page is the view for the single story about a component
#[component]
pub fn Page<S: 'static + Story + Default + Copy + ThreadSafe>(
    /// Phantom data of the story
    #[prop(optional)]
    _story: PhantomData<S>
) -> impl IntoView {
    let story = S::default();
    let canvas = NodeRef::new();

    let tabs: Vec<Box<dyn Tab<SidePanelTabs> + 'static>> = vec![
        Box::new(DescriptionTab { text: story.description() }),
        Box::new(TestsTabs{ story, canvas })
    ];

    let selector = URwSignal::new(SidePanelTabs::Description);

    view! { 
        <>
            <ComponentPanel>
                <Canvas story=story node_ref=canvas />
                <ControlPane story=story />
            </ComponentPanel>
            <div class="flex flex-col basis-1/3 first:basis-1/1 px-4 py-4 overflow-auto print:hidden print:basis-0 min-w-xs w-xs shrink-0 @md:shrink-1">
                <TabPanel
                    id="side-panel"
                    tabs
                    selector
                />
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
        (view!{
            <Description text=self.text />
        }).into_any()
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
            let tests = plays.
                iter().
                enumerate().
                map(|(idx, _play)| {
                    TestView::new(story, idx, canvas)
                }).
                collect::<Vec<_>>();

            let views = tests.iter().
                map(|test|{
                    view!{
                        <Details details={test} />
                    }
                }).
                collect_view();



            (view!{
                <div class="flex-row basis-1/3 first:basis-1/1 px-4 py-4 overflow-auto"> 
                    {views}    
                </div>
            }).into_any()
        }
        else {
            (view!{
                <div class="flex-row basis-1/3 first:basis-1/1 px-4 py-4 overflow-auto"> 
                    <div class="leptos-forge-message-box">No tests were defined!</div>
                </div>
            }).into_any()
        }
    }
}