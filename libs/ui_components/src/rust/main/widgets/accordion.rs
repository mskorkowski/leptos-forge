//! Accordion component

use leptos::prelude::*;

use crate::widgets::details::Details;

use super::details::DetailsParts;

#[component]
pub fn Accordion<'details>(
    /// The list of elements in the accordion
    items: &'details Vec<Box<dyn DetailsParts>>,
) -> impl IntoView {
    let details_list = items
        .iter()
        .map(|item| {
            view! {
                <Details details=item />
            }
        })
        .collect_view();

    view! {
        <div class="leptos-forge-accordion">
            {details_list}
        </div>
    }
}
