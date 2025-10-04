//! Stories for primitive components
//! 

pub mod button;
pub mod label;
pub mod markdown;
pub mod menu;
pub mod switch;

use leptos::prelude::*;

use button::BasicButtonStory;
use forge::RouteDef;
use forge::Section;
use label::BasicLabelStory;
use markdown::MarkdownBaseStory;
use switch::BasicSwitchStory;


/// description of the primitives
const PRIMITIVES_DESC: &str = r############"
# Primitives

Primitives are basic components which will be a basic building block of the all other elements
in the applications. Rarely do they exist independently of other components.

> [!IMPORTANT]  
> Some of primitives are html nodes order dependant. For example a `<Label>` must be placed after an `<Input>`.

"############;

/// Primitives section in the menu
#[derive(Debug,Default,Clone,Copy)]
pub struct Primitives;

impl Section for Primitives {
    fn description(&self) -> &'static str {
        PRIMITIVES_DESC
    }

    fn subroutes(&self) -> Vec<RouteDef> {
         vec![
            RouteDef::page::<BasicButtonStory>("button", "Button"),
            RouteDef::story::<BasicLabelStory>("label", "Label"),
            RouteDef::story::<MarkdownBaseStory>("markdown", "Markdown"),
            RouteDef::Route
            {
                path: "menu",
                label: "Menu",
                component: || view!{"Menu"}.into_any(),
                embedded: |_,_,_| view!{"Embedded menu"}.into_any(),
                subroutes: vec![],
            },
            RouteDef::story::<BasicSwitchStory>("switch", "Switch"),
        ]
    }
}