//! Toggle which allows users to switch between two states `on` and `off`
//!

use leptos::ev::AnimationEvent;
use leptos::html::Button;
use leptos::prelude::*;
use utils_leptos::css::use_add_class;
use utils_leptos::css::use_remove_class;
use utils_leptos::css::use_swap_class;
use utils_leptos::signal::URwSignal;

/// Toggle which allows users to switch between two states `on` and `off`
///
/// ## Anatomy
///
/// Switch has following anatomy*
///
/// ```html
/// <Switch id={id} />
///   <Thumb id={id-switch} />
///   <input type="checkbox" id={id-checkbox} />
/// </Switch>
/// ```
///
/// _* These are not real leptos component but gives an overview of how it's organized internally if you need it_
#[component]
pub fn Switch<S: ToString>(
    /// The id of the toggle element
    ///
    /// See the documentation of the [Switch][Switch#anatomy] section to see how it will be used
    id: S,
    /// The value of the toggle
    #[prop(into)]
    value: URwSignal<bool>,
) -> impl IntoView {
    let id: String = id.to_string();
    let id_thumb: String = format!("{id}-thumb");
    let id_checkbox: String = format!("{id}-checkbox");

    let read: Signal<bool> = value.read_only();
    let ref_button: NodeRef<Button> = NodeRef::<Button>::new();

    let checked: URwSignal<&'static str> = value.map(
        move |v| {
            if *v { "checked" } else { "" }
        },
        |v, new_value| {
            *v = new_value == "checked";
        },
    );

    let on_click = move |_| {
        let current = value.get_untracked();
        use_remove_class(ref_button, ("bg-right", "bg-left"));
        value.set(!current);
    };

    Effect::new(move |old_value: Option<()>| {
        let _ = ref_button.get();
        if read.get() {
            if old_value.is_none() {
                use_add_class(ref_button, "bg-right");
            } else {
                use_swap_class(
                    ref_button,
                    "animate-background-slide-right",
                    "animate-background-slide-left",
                );
            }
        } else if old_value.is_none() {
            use_add_class(ref_button, "bg-left");
        } else {
            use_swap_class(
                ref_button,
                "animate-background-slide-left",
                "animate-background-slide-right",
            );
        }
    });

    let on_animationend = move |_: AnimationEvent| {
        if read.get_untracked() {
            use_swap_class(
                ref_button,
                (
                    "animate-background-slide-left",
                    "animate-background-slide-right",
                    "bg-left",
                ),
                "bg-right",
            );
        } else {
            use_swap_class(
                ref_button,
                (
                    "animate-background-slide-left",
                    "animate-background-slide-right",
                    "bg-right",
                ),
                "bg-left",
            );
        }
    };

    view! {
        <button id={id}
            class="leptos-forge-switch inline-block w-[66px] h-[32px] border-2 border-forgeblue-800 rounded-full cursor-pointer transition-colors bg-linear-[90deg,var(--color-forgegray-800)_0%,var(--color-forgegray-800)_25%,var(--color-forgeblue-500)_75%,var(--color-forgeblue-500)_100%] to=100% bg-[length:400%_100%] forge-animate-keep-both forge-animate-300 focus:border-forgeblue-500 focus:outline-none"
            node_ref = ref_button
            on:click = on_click
            on:animationend = on_animationend
        >
            <div id={id_thumb}
                inner_html=||{"&nbsp;"}
                class="size-[28px] rounded-full bg-forgegray-100 transform transition-transform"
                style:translate= move || {
                    if read.get() {
                        "34px 0px"
                    }
                    else {
                        "0px 0px"
                    }
                }
            />
            <input type="checkbox" id={id_checkbox} class="hidden" prop:checked=checked />
        </button>
    }
}
