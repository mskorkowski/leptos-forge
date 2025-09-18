//! Stories for primitive components
//! 

pub mod button;
pub mod label;
pub mod markdown;
pub mod menu;
pub mod switch;

use forge::Section;

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
}