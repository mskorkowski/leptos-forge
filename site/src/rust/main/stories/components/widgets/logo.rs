//! Stories about logo 
//! 

use leptos::prelude::*;

use ui_components::widgets::field::BlobFile;
use ui_components::widgets::field::TextField;
use ui_components::widgets::logo::Logo;
use utils_leptos::signal::URwSignal;

use forge::Story;

/// Description of the label primitive
const LABEL_DESC: &str = r############"
# Logo
 
The `Logo` component is used to display page/company logo in the top left corner of the page (and maybe other places too)

"############;

/// story describing the basic label behavior
#[derive(Clone, Copy, Debug)]
pub struct BasicLogoStory {
    /// Signal used to set the value of the label
    label: URwSignal<String>,
    /// Value of text input
    file: URwSignal<String>,
}

impl Default for BasicLogoStory {
    fn default() -> Self {
        let label: URwSignal<String> = URwSignal::new("Alternative text about logo".to_string());
        let file: URwSignal<String> = URwSignal::new(String::new());
        BasicLogoStory{
            label,
            file
        }
    }
}

impl Story for BasicLogoStory {
    fn view(&self) -> impl IntoView {
        let label: Signal<String> = self.label.into();
        let file: Signal<String> = self.file.into();

        view! {
            <div class="relative">
                <Logo src=file alt=label />
            </div>
        }
    }

    fn controls(&self) -> impl IntoView {
        let label: URwSignal<String> = self.label;
        let file: URwSignal<String> = self.file;

        view! {
            <BlobFile id="leptos-forge-1-image-upload" label="Select an image" file=file/>
            <TextField text=label label="Alternative text" id="leptos-forge-2-alt-text"/> 
        }
    }

    fn description(&self) -> &'static str {
        LABEL_DESC
    }
}