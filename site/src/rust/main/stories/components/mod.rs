//! Stories related to development of the `leptos_forge` itself

mod primitives;
mod widgets;

use forge::Section;
use leptos::prelude::*;
use primitives::button::BasicButtonStory;
use primitives::markdown::KbdStory;
use primitives::switch::BasicSwitchStory;
use primitives::switch::ToggledOnSwitchStory;
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
        RouteDef::story::<BasicLabelStory>("label", "Label", &[
            RouteDef::page::<InlineLabelStory>("inline", "InlineLabel"),
        ]),
        RouteDef::story::<MarkdownBaseStory>("markdown", "Markdown", &[
            RouteDef::page::<MarkdownAdmonishStory>("admonishes", "Admonishes"),
            RouteDef::page::<MarkdownTableStory>("tables", "Tables"),
            RouteDef::page::<KbdStory>("kbd", "Kbd tag"),
        ]),
        RouteDef::Route
        {
            path: "menu",
            label: "Menu",
            component: || view!{"Menu"}.into_any(),
            subroutes: &[],
        },
        RouteDef::story::<BasicSwitchStory>("switch", "Switch", &[
            RouteDef::page::<ToggledOnSwitchStory>("toggled-on", "Toggled on"),
        ]),
    ]),
    RouteDef::section::<Widgets>("widgets", "Widgets", &[
        RouteDef::story::<BasicCodeareaStory>("codearea", "Codearea", &[
            RouteDef::page::<NonemptyCodeareaStory>("nonempty", "Nonempty"),
        ]),
        RouteDef::page::<BasicLogoStory>("logo", "Logo"),
        RouteDef::story::<BasicTextFieldStory>("text_field", "TextField", &[
            RouteDef::page::<NonemptyTextFieldStory>("nonempty", "Nonempty"),
        ]),
        RouteDef::story::<BasicPasswordFieldStory>("password", "Password", &[
            RouteDef::page::<NonemptyPasswordFieldStory>("nonempty", "Nonempty"),
            RouteDef::page::<NonemptyVisiblePasswordFieldStory>("visible", "Visible"),
        ]),
        RouteDef::story::<BasicTextareaStory>("textarea", "Textarea", &[
            RouteDef::page::<NonemptyTextareaStory>("nonempty", "Nonempty"),
        ]),
        // RouteDef::story::<BasicSingleSelectStory>("single_select", "SingleSelect", &[
        //     RouteDef::page::<ForceOpenSingleSelectStory>("force_open", "Force open"),
        //     RouteDef::page::<ComponentWithAStoreStory>("store_failure", "Store failure"),
        // ]),
    ]),
];

/// description of the [Components] section
const COMPONENTS_DESCRIPTION: &str = r############"
# Components

Components in this section are part of the `leptos_forge_ui_components` library and are specifically designed 
for use within `leptos_forge` and **control panels** of your widgets. They allow you to build control 
interfaces without tying `leptos_forge` to any particular UI framework beyond `leptos`, giving you flexibility 
to choose your preferred UI library elsewhere in your application.

> [!IMPORTANT]
> 
> Please don't treat the documentation of components in this section and it's subpages as remotely "good".
> We are planning to make this documentation presentable but it takes time to get here, since documentation
> for basic concepts is still under development. 
>
> After finishing these GH tickets we will take a serious effort to revamp this corner of documentation:
>
> - [Core documentation](https://github.com/mskorkowski/leptos-forge/issues/3)
> - [Better UX/UI](https://github.com/mskorkowski/leptos-forge/issues/9)
> - [Better DX](https://github.com/mskorkowski/leptos-forge/issues/23)

> [!NOTE]
> 
> **Usage Restriction:** These components are **only intended for use in `leptos_forge`-based applications`** 
> especially the control panels for your components and `leptos_forge` itself.
>
> If you use these components outside of this scope, you bind yourself to a specific version of `leptos_forge`

"############;

/// Components section
/// 
/// This section describes the embedded components from the `leptos_forge_ui_components` crate
#[derive(Debug, Default, Clone, Copy)]
pub struct Components;

impl Section for Components {
    fn description(&self) -> &'static str {
        COMPONENTS_DESCRIPTION
    }
}