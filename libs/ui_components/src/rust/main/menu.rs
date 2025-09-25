//! Contains the components related to creating menus

#![allow(missing_docs)] // Waiting for https://github.com/leptos-rs/leptos/issues/4261

use std::mem::MaybeUninit;

use leptos::prelude::*;
use leptos::ev::MouseEvent;
use leptos::leptos_dom::logging::console_log;
use leptos_node_ref::AnyNodeRef;
use leptos_router::components::A;
use reactive_stores::Patch;
use reactive_stores::PatchField;
use reactive_stores::Store;
use reactive_stores::StorePath;
use utils::prelude::ThreadSafe;
use utils_leptos::css::use_add_class;
use utils_leptos::css::use_remove_class;
use utils_leptos::stores::stored_ref::StoredRef;

/// Menu component
#[component]
pub fn Menu(
    /// Elements in the menu
    /// 
    /// Each element of the menu will be wrapped with a `li` tag. So if you would like to 
    /// create a complex element with then you must wrap it so it will have single parent.
    children: ChildrenFragment
) -> impl IntoView {
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! { <li>{child}</li> })
        .collect::<Vec<_>>();

    view!{
        <nav class="mt-4">
          <ul>{ children }</ul>
        </nav>
    }
}

/// Navigation item in the menu
#[component]
pub fn Navigate<'a, S: ToString + ThreadSafe + Clone>(
    /// path part of the url
    to: S,
    /// label to be shown in the menu
    label: &'static str,
    /// class to add to the menu item
    class: &'static str,
    /// Current location of the browser window
    location: &'a str,
    /// current location
    store: Store<MenuState>,
) -> impl IntoView {
    
    let div = AnyNodeRef::new();

    let class = {
        let to = to.clone();
        let to_string = to.to_string();

        #[allow(unused_assignments)] // this warning is
                                     // 
                                     // 1. related to the effect variable, and talks about it not being read in the
                                     //    Navigate component function itself
                                     // 2. false since we move effect into the effect callback

        if to_string == location {
            store.last_selected_item().patch(
                Some(LastSelectedItem{
                    to: to_string,
                    menu_item: div.into()
                })
            );

            // Safety: This should be hacky but safe way  since we are reading the `effect` variable
            // only after moving it into the effect callback
            #[allow(unsafe_code)]
            let mut effect: Effect<LocalStorage> = unsafe{ *MaybeUninit::zeroed().assume_init_mut() };

            effect = {
                Effect::watch(
                    move || div.get(), 
                    move |element, _, _| {
                        console_log("Fire the effect!");

                        if element.is_some() {
                            console_log("Element is set");
                            let last_selected_item = store.last_selected_item().get_untracked();
                            let to_string = to.to_string();
                            if let Some(last_selected_item) = last_selected_item &&
                                last_selected_item.to == to_string 
                            {
                                console_log("paths are matching!");
                                store.last_selected_item().patch(
                                    Some(LastSelectedItem{
                                        to: to_string,
                                        menu_item: div.into()
                                    })
                                );

                                effect.stop();
                            }

                        }
                    }, 
                    true
                )
            };

            format!("{class} forge-menu-highlight")
        }
        else {
            class.to_string()
        }
    };

    let click = {
        let to = to.clone();
        move |_: MouseEvent| {
            let last_selected_item = store.last_selected_item().get_untracked();
            let to = to.to_string();

            if let Some(last_selected_item) = last_selected_item {
                if last_selected_item.to != to {
                    use_remove_class(last_selected_item.menu_item, "forge-menu-highlight");
                    use_add_class(div, "forge-menu-highlight");

                    store.last_selected_item().patch(
                        Some(LastSelectedItem{
                            to,
                            menu_item: div.into()
                        })
                    );
                }
            }
            else {
                use_add_class(div, "forge-menu-highlight");

                store.last_selected_item().patch(
                    Some(LastSelectedItem{
                        to,
                        menu_item: div.into()
                    })
                );
            }
        }
    };

    view! {
        <div class=class node_ref=div>
            <A href={to.to_string()} on:click=click>{label}</A>
        </div>
    }
}

/// The header in the menu
#[component]
pub fn MenuHeader(label: &'static str, class: &'static str) -> impl IntoView {
    let class = format!("{class} pt-4 font-bold");

    view!{
        <div class=class>{label}</div>
    }
}

/// Last item which was selected from the menu
#[derive(Debug, Clone)]
struct LastSelectedItem{
    /// Path where last selected item was routing
    /// 
    /// We use it to detect the case when user is routing second time to the same path
    /// so we don't blink the highlighting
    to: String,
    /// Node reference to the last selected item in the menu.
    /// 
    /// This allows us to adjust the classes of the menu item without knowing which exactly
    /// element we are dealing with (as long, as we are not messing with the store state)
    menu_item: StoredRef,
}

impl PatchField for LastSelectedItem {
    fn patch_field(
        &mut self,
        new: Self,
        path: &StorePath,
        notify: &mut dyn FnMut(&StorePath),
    ) {
        if self.to != new.to || self.menu_item == StoredRef::Empty {
            *self = new;
            notify(path);
        }
    }
}

/// State of the menu

#[derive(Debug, Store, Patch)]
pub struct MenuState{
    /// Provides information about the last item which was selected in the menu
    last_selected_item: Option<LastSelectedItem>,
}

impl MenuState {
    /// Create a new instance of [MenuState]
    #[allow(clippy::new_without_default)] // I expect the `new` to grow with menu configuration so let's skip this
    pub fn new() -> Self {
        Self{
            last_selected_item: None,
        }
    }
}