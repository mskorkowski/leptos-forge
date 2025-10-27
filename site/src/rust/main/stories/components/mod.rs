//! Stories related to development of the `leptos_forge` itself

mod primitives;
mod widgets;

use forge::Section;
use forge::navigation::RouteDef;
use primitives::Primitives;
use widgets::Widgets;

/// description of the [Components] section
const COMPONENTS_DESCRIPTION: &str = r############"
# Components

> [!WARNING]
>
> ##### 🚧 Work in progress
>
> Currently `leptos_forge_ui_components` only cover really basic use cases and
> there is just a few simple components for now.
>
> You are welcome to create an issue and PR to improve situation.

Components in this section are part of the `leptos_forge_ui_components` library and are specifically designed 
for use within `leptos_forge` and **control panels** of your widgets. They allow you to build control 
interfaces without tying `leptos_forge` to any particular UI framework beyond `leptos`, giving you flexibility 
to choose your preferred UI library elsewhere in your application.

> [!IMPORTANT]
> 
> Please don't treat the documentation of components in this section and it's 
> subpages as remotely "good". We are planning to make this documentation presentable
> but it takes time to get there, especially when documentation for basic concepts
> is still under development.
>
> After finishing these GH tickets we will take a serious effort to revamp this
> corner of documentation:
>
> - [Core documentation](https://github.com/mskorkowski/leptos-forge/issues/3)
> - [Better UX/UI](https://github.com/mskorkowski/leptos-forge/issues/9)
> - [Better DX](https://github.com/mskorkowski/leptos-forge/issues/23)

> [!NOTE]
> 
> **Usage Restriction:** These components are **only intended for use in `leptos_forge`-based applications`** 
> especially the control panels for your components and `leptos_forge` itself.
>
> If you use these components outside of this scope, you bind yourself to
> a specific version of `leptos_forge`

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

    fn subroutes(&self) -> Vec<RouteDef> {
        vec![
            RouteDef::section::<Primitives>("primitives", "Primitives"),
            RouteDef::section::<Widgets>("widgets", "Widgets"),
        ]
    }
}
