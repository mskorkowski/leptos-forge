//! This section describes how to wite interaction tests using [Play][forge::Play]
//! and [Step][forge::Step]

use forge::Section;

/// Description of the [PlaysSection]
const PLAY: &str = r############"
# Play

Play is the instance of the `interaction test`. After component is rendered
user can start a test by pressing the play button. The test simulates the user
interactions with the component.

Plays are integral part of the [story](/documentation/story)

"############;


/// Section about [Play][forge::Play]
#[derive(Debug, Default, Clone, Copy)]
pub struct TestingSection;

impl Section for TestingSection {
    fn description(&self) -> &'static str {
        PLAY        
    }
}