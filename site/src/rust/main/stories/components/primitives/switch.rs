//! Stories for switch primitive

use forge::RouteDef;
use leptos::prelude::*;
use ui_components::primitives::switch::Switch;
use ui_components::widgets::field::SwitchField;
use utils_leptos::signal::URwSignal;

use forge::Story;

/// Description of the switch primitive when toggled of
const SWITCH_DESC: &str = r############"
# Switch

Switch is a component that allows users to toggle between two states. It can be used in forms, settings, and other scenarios where the user needs to make a choice between two options.

"############;

/// Basic switch story
#[derive(Clone, Copy)]
pub struct BasicSwitchStory {
    /// Current state of the switch
    value: URwSignal<bool>,
}

impl Default for BasicSwitchStory {
    fn default() -> Self {
        BasicSwitchStory {
            value: URwSignal::new(false),
        }
    }
}

impl Story for BasicSwitchStory {
    fn description(&self) -> &'static str {
        SWITCH_DESC
    }

    fn controls(&self) -> impl IntoView {
        view! {
            <SwitchField
                id="switch-control-item-1"
                label={"Value".to_string()}
                value={self.value}
            />
        }
    }

    fn view(&self) -> impl IntoView {
        view! {
            <Switch
                id="switch-basic"
                value={self.value}
            />
        }
    }

    fn subroutes(&self) -> Vec<RouteDef> {
        vec![RouteDef::story::<ToggledOnSwitchStory>(
            "toggled-on",
            "Toggled on",
        )]
    }
}

/// Description of the switch primitive when toggled on
const SWITCH_DESC_TOGGLE_ON: &str = r############"
# Switch
# Toggle on

Switch is a component that allows users to toggle between two states. It can be used in forms, settings, and other scenarios where the user needs to make a choice between two options.

"############;

/// Basic switch story
#[derive(Clone, Copy)]
pub struct ToggledOnSwitchStory {
    /// Current state of the switch
    value: URwSignal<bool>,
}

impl Default for ToggledOnSwitchStory {
    fn default() -> Self {
        ToggledOnSwitchStory {
            value: URwSignal::new(true),
        }
    }
}

impl Story for ToggledOnSwitchStory {
    fn description(&self) -> &'static str {
        SWITCH_DESC_TOGGLE_ON
    }

    fn controls(&self) -> impl IntoView {
        view! {
            <SwitchField
                id="switch-control-item-1"
                label={"Value".to_string()}
                value={self.value}
            />
        }
    }

    fn view(&self) -> impl IntoView {
        view! {
            <Switch
                id="switch-on"
                value={self.value}
            />
        }
    }
}
