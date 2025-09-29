//! Section describing URwSignal
//! 

use forge::Section;

/// URwSignal description
const URW_SIGNAL: &str = r############"
# URwSignal

"############;

/// Section describing [URwSignal]
#[derive(Debug, Default, Copy, Clone)]
pub struct URwSignalSection;

impl Section for URwSignalSection {
    fn description(&self) -> &'static str {
        URW_SIGNAL
    }
}