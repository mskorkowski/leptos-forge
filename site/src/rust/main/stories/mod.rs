//! Main entrypoint to the `leptos_forge` site
mod components;

use components::Components;
use forge::RouteDef;


/// Top level routes for the leptos_forge site
pub const ROUTES: &[RouteDef] = &[
    RouteDef::section::<Components>("development", "Development", components::ROUTES),
];