//! Story describing the color palette
//! 

use forge::RouteDef;
use forge::Section;
use forge::Story;

use leptos::prelude::*;
use reactive_stores::Store;
use ui_components::primitives::switch::Switch;
use ui_components::widgets::color_pallete::ColorCode;
use ui_components::widgets::color_pallete::ColorPallete;
use ui_components::widgets::color_pallete::Grid;
use utils_leptos::signal::URwSignal;

use crate::State;

/// Description of the collor pallete widget
const COLOR_PALETTE_DESC: &str = r############"
# Color palette
 
Almost any UI/UX documentation starts with color palette. This widget allow you
to show it inside of the sections of your application.



"############;

/// story describing the basic label behavior
#[derive(Clone, Copy, Debug, Default)]
pub struct ColorPaletteSection {}

impl Section for ColorPaletteSection {
    type Data = State;

    fn description(&self) -> &'static str {
        COLOR_PALETTE_DESC
    }

    fn subroutes(&self) -> Vec<RouteDef<Self::Data>> {
        vec![
            RouteDef::story::<ColorPaletteGridStory>("grid", "Grid"),
        ]
    }
}

/// Color pallete grid example
const COLOR_PALETTE_GRID_DESC: &str = r############"
# Color palette
# Grid

This example shows the grid version of the color pallete. It's useful when color
palette in two dimensions because the range is more complex.

> **Marek**: is using them in case of metallic colors in the UI



"############;

/// story describing the basic label behavior
#[derive(Clone, Copy, Debug, Default)]
pub struct ColorPaletteGridStory {
    /// Wherever we should show color names
    show_names: URwSignal<bool>,
}

impl Story for ColorPaletteGridStory {
    type Data = State;

    fn description(&self) -> &'static str {
        COLOR_PALETTE_GRID_DESC
    }

    
    fn view(&self, _: Store<Self::Data>) -> impl IntoView {
        use ColorCode::*;

        let grid = Grid::new(2, vec![
            Color("red".to_string()),    Color("salmon".to_string()),
            Color("green".to_string()),  Color("lightgreen".to_string()),
            Color("blue".to_string()),   Color("lightblue".to_string()),
        ]);

        view!{
            <ColorPallete colors=grid show_color_name={self.show_names}/>
        }
    }

    fn controls(&self, _: Store<Self::Data>) -> impl IntoView {
        view!{
            <Switch id="color-pallete-show-color-names" value={self.show_names} />
            
        }
    }
}