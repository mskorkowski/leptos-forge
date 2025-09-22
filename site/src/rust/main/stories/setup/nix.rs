//! Contains a section describing nix flake configuration to install rust and all required dependencies for `leptos_forge`
//! 

use forge::Section;

/// description of the [Resources] section
const NIX: &str = r############"
# Resources
"############;


/// Setup section
#[derive(Debug, Default, Clone, Copy)]
pub struct Nix;

impl Section for Nix {
    fn description(&self) -> &'static str {
        NIX
    }
}
