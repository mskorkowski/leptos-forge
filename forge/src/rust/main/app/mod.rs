//! Base component of the application

mod configuration;

use std::fmt::Debug;

use leptos::prelude::*;
use leptos::tachys::view::iterators::StaticVec;
use leptos_node_ref::AnyNodeRef;
use leptos_router::components::Router;
use leptos_router::components::Routes;
use reactive_stores::PatchField;
use reactive_stores::Store;
use ui_components::menu::MenuState;
use ui_components::menu::MenuStateStoreFields;
use ui_components::menu::Header;
use ui_components::model::Visibility;
use ui_components::widgets::logo::Logo;
use utils::prelude::ThreadSafe;
use utils_leptos::signal::URwSignal;


use crate::views::menu_panel::MenuPanel;

use super::navigation::PathSpec;
use super::navigation::RouteDef;
use ui_components::layout::root::Root;
use ui_components::menu::Menu;

use super::views::content::Content;

pub use configuration::*;

/// Classes for the logo element
const LOGO_CLASSES: &str = "\
    max-w-(--forge-main-menu-width) \
    peer-checked:hidden \
";

/// Main application component
#[component]
pub fn App<Data>(
    /// The routing information for the Leptos Forge
    routes: Vec<RouteDef<Data>>,
    /// configuration of the leptos_forge application
    #[prop(into, default=Store::new(LeptosForgeConfiguration::default()),optional)]
    configuration: Store<LeptosForgeConfiguration>,
    /// Initial state of the application store
    #[prop(default=Store::new(Data::default()),optional)]
    store: Store<Data>,
) -> impl IntoView 
where 
    Data: PatchField + Debug + Default + ThreadSafe
{
    let menu_parent_ref = AnyNodeRef::new();
    let menu = Store::new(MenuState::new());
    let menu_visibility: URwSignal<Visibility> = menu.visibility().into();

    let menu_defs = {
        let routes = routes.clone();
        move || {
            

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
                    .flat_map(|route| route.as_routes(PathSpec::Root, store, configuration).into_iter())
                    .collect::<Vec<_>>(),
            )
        }
    };

    let (set_is_routing, set_is_routing_setter) = signal(false);

    // Whenever we are navigating away we should reset the ui visibility
    Effect::new(move || {
        if set_is_routing.get() {
            configuration.menu().visible().set(true);
            configuration.control_panel().visible().set(true);
            configuration.documentation_panel().visible().set(true);
        }
    });

    view! {
        <Router
            set_is_routing=set_is_routing_setter
        >
            <Root>
                <MenuPanel
                    menu_parent_ref
                    configuration
                >
                    <Menu
                        parent=menu_parent_ref
                        visibility=menu_visibility
                        children=ToChildren::to_children(menu_defs)
                    >
                        <Header slot>
                            { move || {
                                let logo_config = configuration.logo();
                                if let Some(logo) = &logo_config.path().get() {
                                    view!{ 
                                        <Logo
                                            id="leptos-forge-logo"
                                            src={logo.to_string()}
                                            alt={logo_config.alt().get().unwrap_or_else(|| "Logo".to_string())}
                                            attr:class=LOGO_CLASSES
                                        /> 
                                    }.into_any()
                                }
                                else {
                                    ().into_any()
                                }
                            }}
                        </Header>
                    </Menu>
                </MenuPanel>
                <Content>
                    <Routes fallback=|| "404" children=ToChildren::to_children(route_defs) />
                </Content>
            </Root>
        </Router>
    }
}
