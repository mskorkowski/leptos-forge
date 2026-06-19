//! Color palette widget showing a given set of colors in a grid
//! 

mod model;
mod select;
mod stores;

use leptos::IntoView;
use leptos::component;
use leptos::prelude::*;
use leptos::view;

pub use model::*;


/// Color palette
#[component]
pub fn ColorPalette(
    /// Grid with colors
    colors: Grid,
    /// Signal deciding wherever we show color names
    #[prop(into)]
    show_color_name: Signal<bool>,
) -> impl IntoView {
    view! {
        <div class="leptos-forge-colorpalette scrollbox h-full min-h-[300px] align-top">
            <div class="scrollable">
            { move || {
                let mut view = vec![];
                for rows in colors.iter() {
                    let mut row = vec![];
                    for cell in rows {
                        let html = view!{
                            <Color color={cell.clone()} show_color_name />
                        }.into_any();

                        row.push(html);
                    }
                    let html = view!{
                        <div class="leptos-forge-colorpalette-row relative h-[100px] align-top">
                            {row}
                        </div>
                    }.into_any();
                    view.push(html);
                }
                view
            }}
            </div>
        </div>
    }
}

/// Shows single color in the color palette
#[component]
fn Color(
    /// Show color name
    #[prop(into)]
    show_color_name: Signal<bool>,
    /// Color code to be shown
    color: ColorCode,
) -> impl IntoView {
    use ColorCode::*;

    let basic_css = "leptos-forge-colorpalette-color inline-block relative w-[200px] h-[100px] align-top";

    match color {
        CssClass{cls, name, ..} => {
            view!{
                <div class=format!("{} {}", basic_css, cls)>
                    <Show
                        when={move || show_color_name.get()}
                    >
                        <ColorName name={name.clone()} />
                        <span></span>
                    </Show>
                    <span inner_html=||{"&nbsp;"} />
                </div>
            }.into_any()
        }
        Color(color) => {
            let bg_color = color.clone();
            view!{
                <div class=basic_css style=format!("background-color: {}", bg_color)>
                    <Show
                        when=move || show_color_name.get()
                    >
                        <ColorName name={color.clone()} />
                    </Show>
                </div>
            }.into_any()
        }
        Skip => {
            view!{
                <div class=basic_css>
                    <span inner_html=||{"&nbsp;"} />
                </div>
            }.into_any()
        }
    }
}

/// Color name
#[component]
fn ColorName(
    /// name of the color
    name: String,
) -> impl IntoView {
    view!{
        <div class="leptos-forge-colorpalette-color-name text-center text-white forge-text-outline-forgegray-950 forge-text-outline-[0.06em] text-2xl font-bold align-top">
            { name }
        </div>
    }
}