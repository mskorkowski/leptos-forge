//! Widget with company logo
use leptos::prelude::*;

use reactive_stores::PatchField;
use reactive_stores::Store;
use utils_leptos::signal::URwSignal;

/// Position of an image
#[derive(Debug, Clone, Copy, PartialEq, Store, Default)]
pub enum Position{
    /// Translation vector (x,y) in `px` for an image
    Translation(i32, i32),
    /// Relative translation in percent of width/height of an image
    Relative(f32, f32),
    /// Initial value
    /// 
    /// Default value for the css property `object-position`
    #[default]
    Initial
}

impl Position {
    /// Returns a value of `object-position` for given value
    pub fn to_css(&self) -> String {
        use  Position::*;
        
        match self {
            Translation(x, y) => format!("{x}px {y}px"),
            Relative(px, py) => format!("{px}% {py}%"),
            Initial => "initial".to_string()
        }
    }
}

impl PatchField for Position {
    fn patch_field(
            &mut self,
            new: Self,
            path: &reactive_stores::StorePath,
            notify: &mut dyn FnMut(&reactive_stores::StorePath),
            _keys: Option<&reactive_stores::KeyMap>,
    ) {
        if *self != new {
            *self = new;
            notify(path);
        }    
    }
}

/// Company logo
#[component]
pub fn Logo<S1: ToString>(
    /// Id of the logo
    id: S1,
    /// url source of the image
    #[prop(into)]
    src: Signal<String>,
    /// alternative text for the image
    #[prop(into)]
    alt: Signal<String>,
    ///position of an image
    #[prop(into, optional, default=URwSignal::new(Position::Initial).into())]
    position: Signal<Position>,
) -> impl IntoView {
    view! {
        <div
            id={format!("{}-box", id.to_string())}
        >
            <img
                id={id.to_string()}
                src=src 
                alt=alt 
                class="leptos-forge-logo object-cover"
                style:object-position=move || { position.get().to_css() }
            />
        </div>
    }
}
