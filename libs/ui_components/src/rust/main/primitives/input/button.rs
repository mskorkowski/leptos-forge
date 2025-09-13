//! Module contains button primitives which integrates with inputs directly

use leptos::attr::Attribute;
use leptos::ev::MouseEvent;
use leptos::prelude::*;
use leptos_icons::Icon;
use utils::prelude::ThreadSafe;
use utils_leptos::signal::URwSignal;

/// Common class for icons in input buttons
pub fn input_button_icon_class() -> impl Attribute {
    view!{
        <{..} class="leptos-forge-input-button-icon mx-[4px]" />
    }
}

/// Primitive `clear input button` component
#[component]
pub fn ClearInputButton(
    /// Signal to invoke when parent component should perform cleaning operation
    /// 
    /// Clear button will set `true` value if the clearing has been invoked
    /// You can use `false` value to prevent other code from clearing your input field
    /// For working example check
    /// - [TextField][crate::widgets::field::TextField]
    /// - [BlobFileInput][crate::widgets::field::BlobFile]
    /// 
    clear: SignalSetter<bool>,
    /// Signal which toggles the visibility of the `clear input button`
    #[prop(into)]
    show: Signal<bool>,
) -> impl IntoView {
    let on_click = move |e: MouseEvent| {
        clear.set(true);
        e.stop_propagation();
        e.prevent_default();
    };

    let clear_button_display_style = move || {
        if show.get() {
            "inline-block"
        }
        else {
            "none"
        }
    };

    view!{
        <button class="
                hidden
                absolute 
                forge-text-standard
                text-forgegray-800
                bg-forgegray-100
                hover:bg-forgeblue-800 
                hover:text-forgeblue-100 
                border-forgeblue-800 
                justify-center
                items-center
                w-5.5
                h-5.5
                right-1.5
                inset-y-16/100
                appearance-none
                rounded-sm
                cursor-pointer
                leading-0
                active:bg-forgeblue-500
                active:border-forgeblue-500
                active:text-forgeblue-100 
                mt-[calc(var(--spacing)*6.7)]
            " 
            on:click=on_click   
            style:display=clear_button_display_style
        ><Icon icon={icondata::MdiClose} {..input_button_icon_class()}/></button>
    }
}

/// Defines a list of states and how they can be navigated between by clicking the [ToggleInputButton]
pub trait Next {
    /// The next state.
    type State: Next;

    /// Returns the next state
    fn next(&self) -> Self::State;

    /// Icon assigned to this state
    fn icon(&self) -> icondata::Icon;
}


/// Input button which allows toggling between different states
#[component]
pub fn ToggleInputButton<State>(
    /// State of the button
    #[prop(into)]
    state: URwSignal<State>,
    /// Wherever the button should be visible
    #[prop(into)]
    show: Signal<bool>,
) -> impl IntoView 
where
    State: Next<State = State> + Default + Clone + ThreadSafe,
{
    let icon: Signal<icondata::Icon> = Signal::derive(move ||{
        state.get().icon()
    });

    let on_click = move |_| {
        let current_state = state.get();
        let next = current_state.next();
        state.set(next);
    };

    let clear_button_display_style = move || {
        if show.get() {
            "inline-block"
        }
        else {
            "none"
        }
    };

    view!{
        <button class="
                hidden
                absolute 
                forge-text-standard
                text-forgegray-800
                bg-forgegray-100
                hover:bg-forgeblue-800 
                hover:text-forgeblue-100 
                border-forgeblue-800 
                justify-center
                items-center
                w-5.5
                h-5.5
                right-1.5
                inset-y-16/100
                appearance-none
                rounded-sm
                cursor-pointer
                leading-0
                active:bg-forgeblue-500
                active:border-forgeblue-500
                mt-[calc(var(--spacing)*6.7)]
            " 
            on:click=on_click   
            style:display=clear_button_display_style
        >
            <Icon icon={icon} {..input_button_icon_class()}/>
        </button>
    }
}

/// List of states for a button which is shown in the password field
#[derive(Debug, Clone, Copy)]
pub enum PasswordButtonStates {
    /// The password should be visible as the `*` in the password field
    Hidden,
    /// The password should be shown as a plaintext
    Visible,
}

impl Next for PasswordButtonStates {
    type State = Self;

    fn next(&self) -> Self::State {

        use PasswordButtonStates::*;

        match self {
            Hidden => Visible,
            Visible => Hidden,
        }
    }

    fn icon(&self) -> icondata::Icon {
        
        use PasswordButtonStates::*;

        match self {
            Hidden => icondata::MdiEyeClosed,
            Visible => icondata::MdiEyeOutline,
        }
    }
}

impl Default for PasswordButtonStates {
    fn default() -> Self {
        Self::Hidden
    }
}