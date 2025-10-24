//! Widget with company logo

use leptos::IntoView;
use leptos::component;
use leptos::prelude::*;
use leptos::view;

/// Company logo
#[component]
pub fn Logo(
    /// url source of the image
    #[prop(into)]
    src: Signal<String>,
    /// alternative text for the image
    #[prop(into)]
    alt: Signal<String>,
) -> impl IntoView {
    view! {
      <div>
        <img src=src alt=alt />
      </div>
    }
}
