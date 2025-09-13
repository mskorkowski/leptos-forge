//! Screens are a huge chunks of ui composed out of widgets and various components to make sure they fit they role and only it

use forge::Section;

/// description of the primitives
const WIDGETS_DESC: &str = r############"
# Screens

Screens are a huge chunks of ui composed out of widgets and various components to make sure they fit they role and only it

"############;

/// Screens section of the storybook
#[derive(Debug, Default, Clone, Copy)]
pub struct Screens;

impl Section for Screens {
    fn description(&self) -> &'static str {
        WIDGETS_DESC
    }
}