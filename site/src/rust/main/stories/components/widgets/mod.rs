//! Stories about widgets
//!

pub mod codearea;
pub mod color_palette;
pub mod logo;
pub mod password;
pub mod select;
pub mod text_field;
pub mod textarea;

use codearea::BasicCodeareaStory;
use color_palette::ColorPaletteSection;
use forge::RouteDef;
use forge::Section;
use logo::BasicLogoStory;
use password::BasicPasswordFieldStory;
use text_field::BasicTextFieldStory;
use textarea::BasicTextareaStory;

use crate::State;
use crate::stories::components::widgets::select::BasicSingleSelectStory;


/// description of the primitives
const WIDGETS_DESC: &str = r############"
# Widgets

Widgets are basic elements designed to work together seamlessly. They often combine many [primitives](/primitives) together.

Widgets are most useful elements of the design language and can be used to create complex user interfaces.

"############;

/// Widget section on the site
#[derive(Debug, Default, Clone, Copy)]
pub struct Widgets;

impl Section for Widgets {
    type Data = State;

    fn description(&self) -> &'static str {
        WIDGETS_DESC
    }

    fn subroutes(&self) -> Vec<RouteDef<Self::Data>> {
        vec![
            RouteDef::story::<BasicCodeareaStory>("codearea", "Codearea"),
            RouteDef::story::<BasicLogoStory>("logo", "Logo"),
            RouteDef::story::<BasicTextFieldStory>("text_field", "TextField"),
            RouteDef::story::<BasicPasswordFieldStory>("password", "Password"),
            RouteDef::story::<BasicTextareaStory>("textarea", "Textarea"),
            RouteDef::story::<BasicSingleSelectStory>("single_select", "SingleSelect"),
            RouteDef::section::<ColorPaletteSection>("color_pallete", "Color Pallete"),
        ]
    }
}
