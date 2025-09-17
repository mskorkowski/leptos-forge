//! Widget with company logo

use leptos::component;
use leptos::prelude::*;
use leptos::view;
use leptos::IntoView;

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
      <div class="--lf-w-80 --lf-h-12 --lf-bg-forgered-200">
        <img src=src alt=alt />
      </div>
    }
}
