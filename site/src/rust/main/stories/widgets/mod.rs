//! Stories about widgets
//! 

pub mod codearea;
pub mod logo;
pub mod password;
// pub mod select;
pub mod text_field;
pub mod textarea;

use forge::Section;

/// description of the primitives
const WIDGETS_DESC: &str = r############"
# Widgets

Widgets are basic elements designed to work together seamlessly. They often combine many [primitives](/primitives) together.

Widgets are most useful elements of the design language and can be used to create complex user interfaces.

"############;

/// Widget section on the site
#[derive(Debug,Default,Clone, Copy)]
pub struct Widgets;

impl Section for Widgets {
    fn description(&self) -> &'static str {
        WIDGETS_DESC
    }
}