//! This module contains detailed descriptions on how to use `leptos_forge`

pub mod sections;
pub mod stories;
pub mod urwsignal;

use forge::RouteDef;
use sections::SectionsSection;
use stories::testing::TestingSection;
use stories::StorySection;
use urwsignal::URwSignalSection;


/// Usage routes for `leptos_forge` site
pub const ROUTES: &[RouteDef] = &[
    RouteDef::section::<StorySection>("story", "Story", &[
        RouteDef::section::<TestingSection>("testing", "Testing", &[]),
    ]),
    RouteDef::section::<SectionsSection>("section", "Section", &[]),
    RouteDef::section::<URwSignalSection>("urwsignal", "URwSignal", &[]),
];