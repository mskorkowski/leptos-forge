//! Base component of the application

use std::fmt::Debug;

use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::tachys::view::iterators::StaticVec;
use leptos_router::components::Router;
use leptos_router::components::Routes;
use reactive_stores::PatchField;
use reactive_stores::Store;
use ui_components::menu::MenuState;
use utils::prelude::ThreadSafe;

use super::navigation::PathSpec;
use super::navigation::RouteDef;
use ui_components::layout::main_menu::MainMenu;
use ui_components::layout::root::Root;
use ui_components::menu::Menu;
use ui_components::widgets::logo::Logo;

use super::views::content::Content;

/// Main application component
#[component]
pub fn App<Data>(
    /// The routing information for the Leptos Forge
    routes: Vec<RouteDef<Data>>,
    /// Path to image to be used as a logo
    #[prop(default=Option::<&'static str>::None,optional)]
    logo: Option<&'static str>,
    #[prop(default=Data::default(),optional)]
    /// Initial state of the application store
    initial_state: Data,
) -> impl IntoView 
where 
    Data: PatchField + Debug + Default + ThreadSafe
{
    let store = Store::new(initial_state);

    let menu_defs = {
        let routes = routes.clone();
        move || {
            let menu = Store::new(MenuState::new());

            let window = window();
            let location = window.location();
            let path = location.pathname().expect("We are running csr mode. Window should exist, location should exist and pathname should be there");

            StaticVec::from(
                routes
                    .iter()
                    .flat_map(move |route| route.as_menu_items(PathSpec::Root, &path, menu))
                    .collect::<Vec<_>>(),
            )
        }
    };

    let route_defs = {
        move || {
            StaticVec::from(
                routes
                    .iter()
                    .flat_map(|route| route.as_routes(PathSpec::Root, store).into_iter())
                    .collect::<Vec<_>>(),
            )
        }
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
