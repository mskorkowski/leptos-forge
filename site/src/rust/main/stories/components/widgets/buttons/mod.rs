//! Module contains stories related to button widget

mod basic_button;

use forge::RouteDef;
use forge::Section;

use crate::State;
use crate::stories::components::widgets::buttons::basic_button::BasicButtonStory;


const BUTTONS_SECTION: &'static str = r############"
# Buttons

There are three kinds of buttons

- basic clickable buttons
- single select inside of the group
- multiselect inside of the group

All buttons come in four different flavours

- **Primary** - should be used for cases where  there exist natural default choice or for call
  to action buttons in the UI. This kind of the button have a background and border which
  distinguishes it from the rest of the UI.
- **Secondary** - should be used as default kind of the button. It will have a border but the background
  will not be popping up from the rest of the UI.
- **Ghost** - This kind of the button will not have any border or background applied. It should
  be used for cases when border would be an additional distraction (ex. toolbars)

All buttons come in four sizes

- **Big** - Button which is bigger then default. It's useful for call to actions, primary choices,
  etc...
- **Normal** - Default size of the buttons.
- **Small** - smaller then normal. Useful in cases like
  - data dense UI to declutter the interface
  - secondary or destructive actions, so users won't select them by accident
  - constrained screen sizes - when there is not enough visible space on the screen
  - low hierarchy - when you need to deemphasize the presence of the element so it won't
    distract the user from the main objective
  - compact menus - when designing compact menus like dropdowns and popovers
- **Unsized** - This is a special kind of a "size". When using this one you are expected to
  provide your own button size using color schema classes. You should avoid this one as
  much as possible.

> [!NOTE]  
> Examples below doesn't showcase the `Unsized` button since using it basically means that your
> use-case goes beyond whatever this guidelines can offer

## Basic button

<Story of="basic_button" />


"############;


/// Buttons section
#[derive(Debug, Clone, Copy, Default)]
pub struct ButtonsSection;

impl Section for ButtonsSection {
    type Data = State;
    fn description(&self) -> &'static str {
        BUTTONS_SECTION
    }

    fn subroutes(&self) -> Vec<RouteDef<Self::Data>> {
        vec![
          RouteDef::private::<BasicButtonStory>("basic_button", "Basic button"),
        ]
    }
}