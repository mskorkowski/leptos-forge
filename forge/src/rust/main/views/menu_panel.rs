//! Main menu panel on the left hand side

use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use reactive_stores::Store;
use ui_components::layout::main_menu::MainMenu;

use crate::LeptosForgeConfiguration;
use crate::LeptosForgeConfigurationStoreFields;
use crate::MenuConfigurationStoreFields;


/// Left hand side navigation menu
#[component]
pub fn MenuPanel(
    /// configuration of the leptos-forge application
    #[prop(into, default=Store::new(LeptosForgeConfiguration::default()),optional)]
    configuration: Store<LeptosForgeConfiguration>,
    /// Content of the menu panel
    children: Children,
    /// Node ref pointing to the element containing the menu
    /// 
    /// > **Note**: This node ref will be bound to one of the children of the
    /// MenuPanel
    menu_parent_ref: AnyNodeRef,
) -> impl IntoView {

    let display = move || {
        if configuration.menu().visible().get() {
            "flex"
        }
        else {
            "none"
        }
    };

    view!{
        <MainMenu 
            container_ref=menu_parent_ref
            style:display=display
        >
            
            { children() }
        </MainMenu>
    }
}