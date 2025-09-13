//! Stories for various components

mod primitives;
mod screen;
mod widgets;

use leptos::prelude::*;
use primitives::button::BasicButtonStory;
use primitives::switch::BasicSwitchStory;
use primitives::switch::ToggledOnSwitchStory;
use screen::Screens;
use primitives::label::BasicLabelStory;
use primitives::label::InlineLabelStory;
use primitives::markdown::MarkdownAdmonishStory;
use primitives::markdown::MarkdownBaseStory;
use primitives::markdown::MarkdownTableStory;
use primitives::Primitives;
use widgets::codearea::BasicCodeareaStory;
use widgets::codearea::NonemptyCodeareaStory;
use widgets::logo::BasicLogoStory;
use widgets::password::BasicPasswordFieldStory;
use widgets::password::NonemptyPasswordFieldStory;
use widgets::password::NonemptyVisiblePasswordFieldStory;
// use widgets::select::store_failure::ComponentWithAStoreStory;
// use widgets::select::BasicSingleSelectStory;
// use widgets::select::ForceOpenSingleSelectStory;
use widgets::text_field::BasicTextFieldStory;
use widgets::text_field::NonemptyTextFieldStory;
use widgets::textarea::BasicTextareaStory;
use widgets::textarea::NonemptyTextareaStory;
use widgets::Widgets;
use forge::navigation::RouteDef;

/// main menu routes
pub const ROUTES: &[RouteDef] = &[
    RouteDef::section::<Primitives>("primitives", "Primitives", &[
        RouteDef::page::<BasicButtonStory>("button", "Button"),
        RouteDef::component::<BasicLabelStory>("label", "Label", &[
            RouteDef::page::<InlineLabelStory>("inline", "InlineLabel"),
        ]),
        RouteDef::component::<MarkdownBaseStory>("markdown", "Markdown", &[
            RouteDef::page::<MarkdownAdmonishStory>("admonishes", "Admonishes"),
            RouteDef::page::<MarkdownTableStory>("tables", "Tables"),
        ]),
        RouteDef {
            path: "menu",
            label: "Menu",
            component: || view!{"Menu"}.into_any(),
            subroutes: &[],
        },
        RouteDef::component::<BasicSwitchStory>("switch", "Switch", &[
            RouteDef::page::<ToggledOnSwitchStory>("toggled-on", "Toggled on"),
        ]),
    ]),
    RouteDef::section::<Widgets>("widgets", "Widgets", &[
        RouteDef::component::<BasicCodeareaStory>("codearea", "Codearea", &[
            RouteDef::page::<NonemptyCodeareaStory>("nonempty", "Nonempty"),
        ]),
        RouteDef::page::<BasicLogoStory>("logo", "Logo"),
        RouteDef::component::<BasicTextFieldStory>("text_field", "TextField", &[
            RouteDef::page::<NonemptyTextFieldStory>("nonempty", "Nonempty"),
        ]),
        RouteDef::component::<BasicPasswordFieldStory>("password", "Password", &[
            RouteDef::page::<NonemptyPasswordFieldStory>("nonempty", "Nonempty"),
            RouteDef::page::<NonemptyVisiblePasswordFieldStory>("visible", "Visible"),
        ]),
        RouteDef::component::<BasicTextareaStory>("textarea", "Textarea", &[
            RouteDef::page::<NonemptyTextareaStory>("nonempty", "Nonempty"),
        ]),
        // RouteDef::component::<BasicSingleSelectStory>("single_select", "SingleSelect", &[
        //     RouteDef::page::<ForceOpenSingleSelectStory>("force_open", "Force open"),
        //     RouteDef::page::<ComponentWithAStoreStory>("store_failure", "Store failure"),
        // ]),
    ]),
    RouteDef::section::<Screens>("components", "Components", &[
    ]),
];