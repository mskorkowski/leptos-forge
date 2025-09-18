//! Stories related to development of the `leptos_forge` itself

mod primitives;
mod widgets;

use forge::Section;
use leptos::prelude::*;
use primitives::button::BasicButtonStory;
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
];

/// description of the [Components] section
const COMPONENTS_DESCRIPTION: &str = r############"
# Embedded components

This part describes the components embedded in the `leptos_forge` itself. You can use them to create
your control panels for your components.

> [!WARNING]  
>
> You should only use this components inside your control panels in the `leptos_forge` based application. 
> They have been created so `leptos_forge` is not dependant on any specific UI library (except for `leptos`),
> so you have a freedom of the choice

# Embedded Control Panel Components

Components in this section are part of the `leptos_forge` library and are specifically designed for use within 
**control panels** of your widgets. They allow you to build control interfaces without tying `leptos_forge` 
to any particular UI framework beyond `leptos`, giving you flexibility to choose your preferred UI library 
elsewhere in your application.

---

### Important Guidelines
> [!NOTE]
> 
> **Usage Restriction:** These components are **only intended for use in control panels** within 
> `leptos_forge`-based applications. Using them outside control panels may lead to unexpected behavior or conflicts.

> [!WARNING]
> 
> These components are tightly coupled with `leptos_forge`'s architecture. For your main application UI, we recommend
> using your preferred framework (e.g., `leptos`, `dioxus`, etc.) to maintain separation of concerns and 
> avoid dependency bloat.

---

### Why Use These Components?
- **Framework Agnosticism:** Ensures `leptos_forge` remains independent of specific UI libraries, keeping your core widget logic decoupled
- **Consistent Experience:** Provides standardized controls that align with `leptos_forge`'s internal architecture
- **Developer Freedom:** Lets you use any UI library (or multiple libraries) in the rest of your application

---

### Quick Start Example
```rust
// In your control panel component
use leptos_forge::controls::{InputNumber, ToggleSwitch};

view! {
  <div class="control-panel">
    <InputNumber label="Size" min=0 max=100 />
    <ToggleSwitch label="Enabled" />
  </div>
}
```

> Replace `dioxus`/`leptos` with your preferred framework for main application UI

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