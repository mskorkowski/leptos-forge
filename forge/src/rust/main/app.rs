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
    routes: &'static [RouteDef]
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
                    <Logo src="/resources/storybook-logo.png" alt="Leptos storybook logo" />
                    <Menu children=ToChildren::to_children(menu_defs) />
                </MainMenu>
                <Content>
                    <Routes fallback=|| "Fallback" children=ToChildren::to_children(route_defs) />
                </Content>
            </Root>
        </Router>
    }
}