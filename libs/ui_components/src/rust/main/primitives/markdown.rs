//! Component rendering a markdown text
//!

use leptos::prelude::*;
use pulldown_cmark::Options;
use pulldown_cmark::Parser;

/// Renders a static markdown text
///
///
#[component]
pub fn Markdown(
    /// raw markdown to be rendered
    #[prop(into)]
    src: Signal<String>,
) -> impl IntoView {
    let mut configuration = Options::empty();
    configuration.insert(Options::ENABLE_TABLES);
    configuration.insert(Options::ENABLE_FOOTNOTES);
    configuration.insert(Options::ENABLE_STRIKETHROUGH);
    configuration.insert(Options::ENABLE_TASKLISTS);
    configuration.insert(Options::ENABLE_GFM);

    let inner_html = move || {
        let source = src.get();
        let parser = Parser::new_ext(&source, configuration);

        // Write to a new String buffer.
        let mut inner_html = String::new();
        pulldown_cmark::html::push_html(&mut inner_html, parser);
        inner_html
    };

    view! {
        <div inner_html=inner_html />
    }
}
