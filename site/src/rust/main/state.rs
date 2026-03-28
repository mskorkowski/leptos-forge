//! Application state
#![allow(missing_docs)]

use reactive_stores::Patch;
use reactive_stores::Store;

/// State of the leptos forge site application
#[derive(Debug, Default, Store, Patch)]
pub struct State {}