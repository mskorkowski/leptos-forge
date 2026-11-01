//! Contains the components related to creating menus

use std::mem::MaybeUninit;

use leptos::ev::MouseEvent;
use leptos::html::Input;
use leptos::html::Nav;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos_node_ref::AnyNodeRef;
use leptos_router::components::A;
use leptos_use::UseIntersectionObserverOptions;
use leptos_use::use_intersection_observer_with_options;
use reactive_stores::Patch;
use reactive_stores::PatchField;
use reactive_stores::Store;
use reactive_stores::StorePath;

use leptos_icons::Icon;


use utils::prelude::ThreadSafe;
use utils_leptos::prelude::*;

use crate::model::Visibility;



/// CSS clases for the menu itself
const MAIN_MENU_CSS_CLASSES: &str = "\
    peer-checked:block \
    mt-4 \
    ml-4 \
    hidden \
    md:block \
    mx-auto \
";

/// CSS of the hamburger icon
const MENU_OPEN_ICON_CSS_CLASSES: &str = "\
    md:hidden \
    peer-checked:hidden \
    ml-auto \
    mr-2 \
";

/// CSS of the hamburger icon
const MENU_CLOSE_ICON_CSS_CLASSES: &str = "\
    hidden \
    md:hidden \
    peer-checked:block \
    place-self-start \
    ml-auto \
    mr-2 \
    mt-[7px] \
";

/// Header of the menu
#[slot]
pub struct Header {
    /// content of the header
    children: ChildrenFn
}

/// Menu component
#[component]
pub fn Menu(
    /// Elements in the menu
    ///
    /// Each element of the menu will be wrapped with a `li` tag. So if you would 
    /// like to create a complex element then you must wrap with a single parent 
    /// node so after wrapping it won't be split between multiple nodes
    children: ChildrenFragment,
    /// Signal controlling the visibility of the menu
    /// 
    /// If you would like to hide the menu after the navigation took place
    /// (user clicked an option in the menu) you should set this signal to 
    /// [Visibility::Hide]. We can't do it on our own because your menu might
    /// be more complex then we can handle (anchors, links to the menu parts, 
    /// navigation across menu, multistage menu, you name it)
    visibility: URwSignal<Visibility>,
    /// Reference to the parent node
    #[prop(optional, default=None)]
    parent: Option<AnyNodeRef>,
    /// Header of the menu
    /// 
    /// By default inside of the header there is a menu toggle when viewing the
    /// menu on the mobile
    #[prop(optional)]
    header: Option<Header>,
) -> impl IntoView {
    let children = children()
        .nodes
        .into_iter()
        .map(|child| view! { <li>{child}</li> })
        .collect::<Vec<_>>();

    let nav_ref = NodeRef::<Nav>::new();
    let input_ref = NodeRef::<Input>::new();

    use_intersection_observer_with_options(
        nav_ref,
        move | entries, _ | {
            for entry in entries {
                if entry.is_intersecting() {
                    visibility.set(Visibility::Visible)
                }
                else {
                    visibility.set(Visibility::Hidden)
                }
            }
        },
        UseIntersectionObserverOptions::default().
            root(parent)
    );

    Effect::new(move |_| {
        let visibility = visibility.get();
        match visibility {
            Visibility::Hide => {
                if let Some(input) = input_ref.get_untracked() {
                    input.set_checked(false);
                }
            },
            Visibility::Show => {
                if let Some(input) = input_ref.get_untracked() {
                    input.set_checked(true);
                }
            },
            Visibility::Visible | Visibility::Hidden => {
                // we don't need to act on the state which declares the end result
            }
        }
    });

    let header = move || header.map(|h| (h.children)());

    let breakpoints = use_breakpoint(UseBreakpointConfiguration::tailwind4());
    let md_breakpoint = breakpoints.breakpoint(Tailwindcss4Breakpoints::M);

    Effect::new(move |_| {
        console_log("breakpoint");
        let _ = md_breakpoint.get();

        if let Some(input) = input_ref.get_untracked() {
            input.set_checked(false);
        }
    });

    view! {
        <input 
            id="leptos-forge-main-menu-toggle"
            node_ref=input_ref
            class="peer hidden"
            type="checkbox"
        />
        { header() }
        <nav 
            node_ref=nav_ref
            class=MAIN_MENU_CSS_CLASSES
        >
          <ul>{ children }</ul>
        </nav>
        <label for="leptos-forge-main-menu-toggle" class=MENU_OPEN_ICON_CSS_CLASSES>
            <Icon icon=icondata::MdiMenu height="3em" width="3em" />
        </label>
        <label for="leptos-forge-main-menu-toggle" class=MENU_CLOSE_ICON_CSS_CLASSES>
            <Icon icon=icondata::MdiMenuClose height="3em" width="3em" />
        </label>
    }
}

/// Navigation item in the menu
#[component]
pub fn Navigate<'a, S: ToString + ThreadSafe + Clone>(
    /// path part of the url
    to: S,
    /// label to be shown in the menu
    label: &'static str,
    /// Label to be used when using assistive technologies when meaning of the
    /// link can be confusing in the context when just listening
    aria_label: Option<&'static str>,
    /// class to add to the menu item
    class: &'static str,
    /// Current location of the browser window
    location: &'a str,
    /// current location
    store: Store<MenuState>,
) -> impl IntoView {
    let div = AnyNodeRef::new();

    let highlight = ("font-bold", "text-forgegray-50", "border-l-forgegray-50");
    let normal = "border-l-forgegray-800";

    let class = {
        let to = to.clone();
        let to_string = to.to_string();

        #[allow(unused_assignments)]
        // this warning is
        //
        // 1. related to the effect variable, and talks about it not being read in the
        //    Navigate component function itself
        // 2. false since we move effect into the effect callback
        if to_string == location {
            store.last_selected_item().patch(Some(LastSelectedItem {
                to: to_string,
                menu_item: div.into(),
            }));

            // Safety: This should be hacky but safe way since we are reading the `effect` variable
            // only after moving it into the effect callback, so it must become initialized before
            // being read
            //
            // Our goal here is to drop the effect after first successful firing since it won't be
            // needed later
            #[allow(unsafe_code)]
            let mut effect: Effect<LocalStorage> =
                unsafe { *MaybeUninit::zeroed().assume_init_mut() };

            effect = {
                Effect::watch(
                    move || div.get(),
                    move |element, _, _| {
                        if element.is_some() {
                            let last_selected_item = store.last_selected_item().get_untracked();
                            let to_string = to.to_string();
                            if let Some(last_selected_item) = last_selected_item
                                && last_selected_item.to == to_string
                            {
                                store.last_selected_item().patch(Some(LastSelectedItem {
                                    to: to_string,
                                    menu_item: div.into(),
                                }));

                                effect.stop();
                                effect.dispose();
                            }
                        }
                    },
                    true,
                )
            };

            format!("{} {} {} {class}", highlight.0, highlight.1, highlight.2)
        } else {
            class.to_string()
        }
    };

    let class = format!("py-1 border-l-forgegray-800 border-l-3 {class}");

    let click = {
        let to = to.clone();
        move |_: MouseEvent| {
            let last_selected_item = store.last_selected_item().get_untracked();
            let to = to.to_string();

            if let Some(last_selected_item) = last_selected_item {
                if last_selected_item.to != to {
                    use_swap_class(last_selected_item.menu_item, highlight, normal);
                    use_swap_class(div, normal, highlight);

                    store.last_selected_item().patch(Some(LastSelectedItem {
                        to,
                        menu_item: div.into(),
                    }));
                }
            } else {
                use_swap_class(div, normal, highlight);

                store.last_selected_item().patch(Some(LastSelectedItem {
                    to,
                    menu_item: div.into(),
                }));
            }

            // Hide menu after navigating
            store.visibility().set(Visibility::Hide);
        }
    };

    let link_class = view! {
        <{..} class="hover:text-forgeblue-300" />
    };

    view! {
        <div class=class node_ref=div>
            <A href={to.to_string()} on:click={click} attr:aria-label={aria_label}  {.. link_class}>{label}</A>
        </div>
    }
}

/// The header in the menu
#[component]
pub fn MenuHeader(
    /// Label of the header
    label: &'static str,
    /// Extra classes for the header
    class: &'static str
) -> impl IntoView {
    let class = format!("{class} ml-6 pb-2 pt-8 font-bold text-forgegray-400");

    view! {
        <div class=class>{label}</div>
    }
}

/// Last item which was selected from the menu
#[derive(Debug, Clone)]
struct LastSelectedItem {
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
        _keys: Option<&reactive_stores::KeyMap>,
    ) {
        if self.to != new.to || self.menu_item == StoredRef::Empty {
            *self = new;
            notify(path);
        }
    }
}

/// State of the menu

#[derive(Debug, Store, Patch)]
pub struct MenuState {
    /// Provides information about the last item which was selected in the menu
    last_selected_item: Option<LastSelectedItem>,
    /// Menu visibility
    ///
    /// By default it's initiated with [Visibility::Hidden]
    /// 
    /// # Accuracy
    /// 
    /// Tracking visibility of element is not a pleasant experience
    /// 
    /// This value might be inaccurate from a point of view in the desktop mode
    /// because we after navigating in general you would request hiding menu
    /// but in desktop mode hiding doesn't do a shit and intersection doesn't
    /// change either so, for some time it might be reported as hidden even when
    /// it's displayed on the left hand side as is
    /// 
    /// Mobile view (where this is really important) works correctly because menu
    /// internally is coupled with a checkbox tracking the visibility of the menu.
    /// So changing a state of the value to an intent ([Visibility::Hide], 
    /// [Visibility::Show]) would toggle the checkbox, which will force the
    /// visibility observer to trigger because state of the menu is bound by the
    /// css classes to the checkbox. This solution is way more reliable the any
    /// js we can cook.
    visibility: Visibility
}

impl MenuState {
    /// Create a new instance of [MenuState]
    #[allow(clippy::new_without_default)] // I expect the `new` to grow with menu configuration so let's skip this
    pub fn new() -> Self {
        Self {
            last_selected_item: None,
            visibility: Visibility::Hidden
        }
    }
}
