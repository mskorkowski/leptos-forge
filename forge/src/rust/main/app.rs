//! Base component of the application

use leptos::prelude::*;
use leptos::tachys::view::iterators::StaticVec;
use leptos_router::components::Routes;
use leptos_router::components::Router;

use super::navigation::PathSpec;
use super::navigation::RouteDef;
use ui_components::widgets::logo::Logo;
use ui_layout::main_menu::MainMenu;
use ui_layout::root::Root;
use ui_components::menu::Menu;

use super::views::content::Content;

/// Main application component
#[component]
pub fn App(
    /// The routing information for the Leptos Forge
    routes: &'static [RouteDef],
    /// Path to image to be used as a logo
    #[prop(default=Option::<&'static str>::None,optional)]
    logo: Option<&'static str>,
) -> impl IntoView {

    let menu_defs = || {
        StaticVec::from(
            routes.
                iter().
                flat_map(|route| {
                    route.as_menu_items(PathSpec::Root)
                }).
                collect::<Vec<_>>()
        )
    };    

    let route_defs = || {
        StaticVec::from( 
            routes
                .iter()
                .flat_map(|route| {
                    route.as_routes(PathSpec::Root).into_iter()
                })
                .collect::<Vec<_>>()
        )
    };

    view! {
        <Router>
            <Root>
                <MainMenu>
                    { move || {
                        if let Some(logo) = &logo {
                            view!{ <Logo src={logo.to_string()} alt="Logo" /> }.into_any()
                        }
                        else {
                            ().into_any()
                        }
                    }}
                    <Menu children=ToChildren::to_children(menu_defs) />
                </MainMenu>
                <Content>
                    <Routes fallback=|| "404" children=ToChildren::to_children(route_defs) />
                </Content>
            </Root>
        </Router>
    }
}