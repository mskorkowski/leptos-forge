//! This module contains detailed descriptions on how to use `leptos_forge`

use forge::RouteDef;
use sections::SectionsSection;

pub mod sections;

/// Usage routes for `leptos_forge` site
pub const ROUTES: &[RouteDef] = &[
    RouteDef::section::<SectionsSection>("section", "Section", &[]),
];