//! Implements a view for the [app::Section]
//! 

mod markdown;

use std::marker::PhantomData;

use leptos::prelude::*;

use crate::Section;

use super::description::Description;

/// Displays a [Section] in the 
#[component]
pub fn Section<S: 'static + Section + Default + Copy + Send>(
    /// Section to be shown
    #[prop(optional)]
    _section: PhantomData<S>
) -> impl IntoView {
    let description = S::default().description();

    

    view! {
        <Description text=description />
    }
}