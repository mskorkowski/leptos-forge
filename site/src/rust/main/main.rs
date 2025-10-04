//! Site for `leptos_forge`
//! 
//! The site is both the website with documentation about the `leptos_forge` and 
//! the example at the same time.
//! 
//! We try to keep it clean, documented and easy to read.

#![doc(test(attr(deny(unused))))]
#![doc(test(attr(deny(dead_code))))]
#![allow(dead_code)]

#![deny(missing_docs)]
#![deny(clippy::empty_docs)]
#![deny(clippy::missing_docs_in_private_items)]
#![deny(clippy::missing_docs_in_private_items)]
#![deny(clippy::missing_safety_doc)]
#![deny(clippy::missing_errors_doc)]
#![deny(clippy::missing_fields_in_debug)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![doc(test(attr(deny(unused))))]

mod stories;

use forge::RouteDef;
use leptos::prelude::*;


use forge::App;
use log::Level;
use stories::components::Components;
use stories::setup::adding_tests::AddingTests;
use stories::setup::nix::Nix;
use stories::setup::refine_story::RefineCounterStory;
use stories::setup::resources::Resources;
use stories::setup::resources::Tailwind;
use stories::setup::Setup;
use stories::usage::routes::RoutesSection;
use stories::usage::sections::SectionsSection;
use stories::usage::stories::StorySection;
use stories::usage::urwsignal::URwSignalSection;
use stories::Main;

/// Entrypoint of the application
pub fn main() {
    _ = console_log::init_with_level(Level::Debug);
    console_error_panic_hook::set_once(); 

    // Top level routes for the leptos_forge site
    let routes = vec![
        RouteDef::section::<Main>("/", "Leptos Forge"),
        RouteDef::header("guides", "GUIDES", vec![
            RouteDef::section::<Setup>("create_project", "Create project"),
            RouteDef::section::<RefineCounterStory>("first_story", "Implement the first story"),
            RouteDef::section::<AddingTests>("adding_tests", "Adding tests"),
            RouteDef::section::<Resources>("resources", "Resources"),
            RouteDef::section::<Tailwind>("tailwind", "Tailwind"),
            RouteDef::section::<Nix>("nix", "Nix"),
        ]),
        RouteDef::header("documentation", "DOCUMENTATION", vec![
            RouteDef::section::<StorySection>("story", "Story"),
            RouteDef::section::<SectionsSection>("section", "Section"),
            RouteDef::section::<RoutesSection>("routes", "Routing"),
            RouteDef::section::<URwSignalSection>("urwsignal", "URwSignal"),
        ]),
        RouteDef::header("development", "DEVELOPMENT", vec![
            RouteDef::section::<Components>("components", "Components")
        ])
    ];

    mount_to_body(move || {
        view!{
            <App routes logo="/resources/leptos_forge/logo/logo.svg" />
        }
    });
}