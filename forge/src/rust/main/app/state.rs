//! Describes the state of the leptos_forge application
//!

use reactive_stores::Patch;
use reactive_stores::Store;

/// The top of the application state
///
/// The main goal is to reactively track
///
/// - settings - to be done
/// - internal state - nothing yet
#[derive(Debug, Store, Patch)]
pub struct State {}

impl State {
    /// Create a new instance of [State]
    pub fn new() -> Self {
        Self {}
    }
}
