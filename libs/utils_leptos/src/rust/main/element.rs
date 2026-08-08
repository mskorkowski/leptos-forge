//! Helper to handle the html elements
//! 

use leptos::prelude::GetUntracked;
use leptos_use::core::ElementMaybeSignal;
use leptos_use::core::IntoElementMaybeSignal;
use web_sys::Element;
use web_sys::ScrollBehavior;
use web_sys::ScrollIntoViewOptions;
use web_sys::ScrollLogicalPosition;

/// Scrolls the nearest scroll container in such a way that `target` is visible
pub fn use_scroll_into_view<E, M>(target: E) 
where
    E: IntoElementMaybeSignal<Element, M>,
    M: Sized,
{
    let target: ElementMaybeSignal<Element> = target.into_element_maybe_signal();
    if let Some(node) = target.get_untracked() {
        let options: ScrollIntoViewOptions = ScrollIntoViewOptions::default();
        options.set_behavior(ScrollBehavior::Smooth);
        options.set_block(ScrollLogicalPosition::Nearest);
        options.set_container(web_sys::ScrollIntoViewContainer::Nearest);
        node.scroll_into_view_with_scroll_into_view_options(&options);

    }
}