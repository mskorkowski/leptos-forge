//! Implements a view for the [app::Section]
//!

mod markdown;

use std::marker::PhantomData;
use std::str::Split;

use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use markdown::MarkdownParser;
use markdown::MarkdownToken;
use ui_components::primitives::markdown::Markdown;

use crate::RouteDef;
use crate::Section;

/// Displays a [Section] in the
#[component]
pub fn Section<S: 'static + Section + Default + Copy + Send>(
    /// Section to be shown
    #[prop(optional)]
    _section: PhantomData<S>,
) -> impl IntoView {
    let section = S::default();
    let description = section.description();
    let parser = MarkdownParser::new();

    console_log("parsing description");

    let tokens = parser.parse(description);

    console_log("rendering tokens");

    let views = tokens
        .into_iter()
        .map(|token| {
            view! {
                <MarkdownTokenView<S> token />
            }
        })
        .collect_view();

    console_log("rendering <Section>");

    view! {
        <div class="leptos-forge-section-container scrollbox w-full flex flex-col">
            <div class="leptos-forge-section markdown scrollable">
                { views }
            </div>
        </div>
    }
}

/// Finds a story in the subroutes tree
fn navigate_subtree<'a>(
    mut path: Split<'a, &'static str>,
    subroutes: &'a Vec<RouteDef>,
) -> Option<&'a RouteDef> {
    let mut next = path.next();
    let mut subroutes = subroutes;

    while let Some(segment) = next {
        if let Some(route) = subroutes.iter().find(|route| route.path() == segment) {
            subroutes = match route {
                RouteDef::Header { subroutes, .. } | RouteDef::Route { subroutes, .. } => subroutes,
            };

            next = path.next();

            if next.is_none() {
                return Some(route);
            }
        } else {
            return None;
        }
    }

    None
}

/// Generates a view for [MarkdownToken] instances
#[component]
fn MarkdownTokenView<S: 'static + Section + Default + Copy + Send>(
    /// Token to be rendered
    token: MarkdownToken<'static>,
    /// Section to be shown
    #[prop(optional)]
    _section: PhantomData<S>,
) -> impl IntoView {
    use MarkdownToken::*;

    let section = S::default();
    let subroutes = section.subroutes();

    match token {
        Header{ level, text, ..  } => {
            let text = text.to_string();
            view!{<MarkdownHeader level text /> }.into_any()
        },
        Story { story: Some(path), controls, .. } => {
            let path: Split<'_, &'static str> = path.split("/");
            if let Some(route) = navigate_subtree(path, &subroutes) {
                match route {
                    RouteDef::Header { .. } => {
                        view!{<Markdown src="> Expected story, but header was found" /> }.into_any()
                    }
                    RouteDef::Route { embedded, .. } => {
                        embedded(true, controls, false)
                    }
                }
            }
            else {
                view!{<Markdown src="> Can't find story to show" /> }.into_any()
            }
        },
        Story{ story: None, .. }  => {
            view!{<Markdown src="> Attribute of was not found in the `<Story />` tag.\n> \n> Add the `of=\"subroute/path\"` to your `<Story />` tag." />}.into_any()
        },
        Markdown { text, .. } => {
            view!{
                <Markdown src={text} />
            }.into_any()
        }
    }
}

/// Markdown header component
///
/// This allows us to remove one level of div and a call to a Markdown parser
/// for every header
#[component]
fn MarkdownHeader(
    /// Level of the header
    ///
    /// - `# Header` - 1
    /// - `## Header` - 2
    /// ...
    level: usize,
    /// Value of the header
    text: String,
) -> impl IntoView {
    match level {
        1 => view! {<h1>{text}</h1>}.into_any(),
        2 => view! {<h2>{text}</h2>}.into_any(),
        3 => view! {<h3>{text}</h3>}.into_any(),
        4 => view! {<h4>{text}</h4>}.into_any(),
        5 => view! {<h5>{text}</h5>}.into_any(),
        _ => view! {<h6>{text}</h6>}.into_any(),
    }
}
