//! Contains the components related to creating menus

use leptos::prelude::*;
use leptos_router::components::A;

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
pub fn Navigate<S: ToString>(
    /// path part of the url
    to: S,
    /// label to be shown in the menu
    label: &'static str,
    /// class to add to the menu item
    class: &'static str,
) -> impl IntoView {
    let to = to.to_string();
    view! {
        <div class=class>
            <A href=to>{label}</A>
        </div>
    }
}

//<Logo />
// <nav class="mt-4">
// <ul>
//   <li>Home</li>
//   <li>Transactions</li>
//   <li>Companies</li>
//   <li>Tools</li>
//   <li>Settings</li>
// </ul>
// </nav>
// <div class="grow" />
// <nav>
// <div>Personal</div>
// <ul>
//   <li>Personal settings</li>
//   <li>Logout</li>
// </ul>
// </nav>