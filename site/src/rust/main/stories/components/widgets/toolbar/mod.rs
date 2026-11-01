//! Stories for the navigation toolbar inspired by the Apple's looking glass
//! 

mod search_button;
mod simple_toolbar;
mod toolbar_with_search;

use forge::RouteDef;
use forge::Section;

use search_button::SearchButtonStory;
use simple_toolbar::SimpleToolbarStory;

use crate::State;
use crate::stories::components::widgets::toolbar::toolbar_with_search::ToolbarWithSearchStory;

/// Description of the toolbar section
const TOOLBAR_DESCRIPTION: &str = r############"
# Bottom navigation toolbar

Bottom navigation toolbar inspired by the Apple's looking glass.

## Search

Search button (if present) in on the right side of the menu as the distinctly
separate element.

After clicking the search button it shows the search input which spans across the
whole toolbar area

Search menu item differs from the other toolbar menu items because it doesn't 
have a label under the magnifying glass icon.

"############;


/// Section about navigation toolbar
#[derive(Debug, Default, Clone, Copy)]
pub struct ToolbarSection {

}

impl Section for ToolbarSection {
    type Data = State;

    fn description(&self) -> &'static str {
        TOOLBAR_DESCRIPTION
    }

    fn subroutes(&self) -> Vec<RouteDef<Self::Data>> {
        vec![
            RouteDef::story::<SimpleToolbarStory>("simple_toolbar", "Simple"),
            RouteDef::story::<SearchButtonStory>("search_item", "Search button"),
            RouteDef::story::<ToolbarWithSearchStory>("toolbar_with_search", "Toolbar with search"),
        ]
    }
}