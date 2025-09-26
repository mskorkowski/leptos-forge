//! Story which shows the support for `<kdb>` tag in Markdown

use leptos::prelude::*;
use forge::Story;
use ui_components::primitives::markdown::Markdown;
use ui_components::widgets::field::Codearea;
use utils_leptos::signal::URwSignal;

/// Description of the [KdbStory] for Markdown
const KBD: &str = r############"
# Markdown
# `<kbd>` tag

You should use the `<kbd>` tag to show the text which is being/should be typed
using keyboard.

For example:

<kbd>Shift</kbd>+<kbd>Enter</kbd> - will submit the form

"############;

/// Story showcasing the `<kbd>` html tag behavior
#[derive(Debug, Clone, Copy)]
pub struct KbdStory{
    /// markdown content be shown to the user
    markdown: URwSignal<String>
}

impl Default for KbdStory {
    fn default() -> Self {
        Self{ 
            markdown: URwSignal::new(
                "Example of the `<kdb>` to showing alt key:\n\n<kbd>alt</kbd>".to_string()
            ) 
        }
    }
}

impl Story for KbdStory {
    fn controls(&self) -> leptos::prelude::AnyView {
        view!{
            <Codearea id="leptos-forge-markdown-demo-textarea" text=self.markdown label={"Markdown document".to_string()} />
        }.into_any()
    }

    fn description(&self) -> &'static str {
       KBD   
    }

    fn view(&self) -> AnyView {
        view! {
            <Markdown src=self.markdown/>
        }.into_any()
    }
}