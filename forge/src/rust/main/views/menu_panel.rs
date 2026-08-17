//! Main menu panel on the left hand side

use leptos::prelude::*;
use reactive_stores::Store;
use ui_components::layout::main_menu::MainMenu;
use ui_components::widgets::logo::Logo;

use crate::LeptosForgeConfiguration;
use crate::LeptosForgeConfigurationStoreFields;
use crate::LogoConfigurationStoreFields;
use crate::MenuConfigurationStoreFields;


/// Left hand side navigation menu
#[component]
pub fn MenuPanel(
    /// configuration of the leptos-forge application
    #[prop(into, default=Store::new(LeptosForgeConfiguration::default()),optional)]
    configuration: Store<LeptosForgeConfiguration>,
    /// Content of the menu panel
    children: Children,
) -> impl IntoView {

     let display = move || {
        if configuration.menu().visible().get() {
            "block"
        }
        else {
            "none"
        }
     };

    view!{
        <MainMenu style:display=display >
            { move || {
                let logo_config = configuration.logo();
                if let Some(logo) = &logo_config.path().get() {
                    view!{ <Logo id="leptos-forge-logo" src={logo.to_string()} alt={logo_config.alt().get().unwrap_or_else(|| "Logo".to_string())} /> }.into_any()
                }
                else {
                    ().into_any()
                }
            }}
            { children() }
        </MainMenu>
    }
}