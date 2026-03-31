//! Color pallete widget showing a given set of colors in a grid
//! 

use leptos::IntoView;
use leptos::component;
use leptos::leptos_dom::logging::console_log;
use leptos::prelude::*;
use leptos::view;

/// Structure representing a grid
pub struct Grid{
    /// Number of column in the grid
    cols: usize,
    /// Data in the grid
    data: Vec<ColorCode>,
}

impl Grid{
    /// Creates new instance of the grid
    /// 
    /// Number of rows is determined automatically by the length of the `data`
    /// argument `ceil(data/cols)`.` If the last row doesn't contain enough data
    /// to fill the entire number of cols, grid will assume that this colors 
    /// should be skipped.
    /// 
    /// # Arguments
    /// 
    /// - **cols** - Number of columns in the grid
    /// - **data** - List of colors, every `cols` of colors is a row in the grid
    /// 
    /// # Examples
    /// 
    /// ## Single row of colors
    /// 
    /// ```rust
    /// use ui_components::widgets::color_pallete::Grid;
    /// 
    /// Grid::new(3, vec![ ColorCode::Color("red"), ColorCode::Color("green"), ColorCode::Color::("blue") ])
    /// ```
    /// 
    /// ## Single column of colors
    /// 
    /// ```rust
    /// use ui_components::widgets::color_pallete::Grid;
    /// 
    /// Grid::new(1, vec![ ColorCode::Color("red"), ColorCode::Color("green"), ColorCode::Color::("blue") ])
    /// ```
    pub fn new(cols: usize, data: Vec<ColorCode>) -> Grid {
        Grid{
            cols,
            data,
        }
    }

    /// Returns the iterator over grid rows
    pub fn iter<'this>(&'this self) -> GridRows<'this> {
        GridRows{ 
            grid: self, 
            position: 0,
        }
    }
}

/// Iterator over columns in a grid row
pub struct GridCols<'grid> {
    /// data to iterate over
    grid: &'grid Grid,
    /// position in the data
    /// 
    /// Iterator will be complete if `position-start >= grid.cols`
    position: usize,
    /// starting position
    start: usize,
}

impl<'grid> Iterator for GridCols<'grid> {
    type Item = &'grid ColorCode;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position - self.start < self.grid.cols && self.position < self.grid.data.len() {
            console_log(&format!("Col {}", self.position - self.start));
            let current= self.grid.data.get(self.position);
            self.position += 1;
            current // we know it's some, by the if statement
        }
        else if self.position - self.start < self.grid.cols {
            console_log(&format!("Col injecting {}", self.position - self.start));
            self.position += 1;
            // we don't have any more data but we need to fill the grid at the end
            Some(&ColorCode::Skip)
        }
        else {
            // End of iterator
            None
        }
    }
}

/// Iterator over rows in the grid
/// 
/// Gives access to [GridCols] iterator
pub struct GridRows<'grid> {
    /// data to iterate over
    grid: &'grid Grid,
    /// current row number
    position: usize,
}

impl<'grid> Iterator for GridRows<'grid> {
    type Item = GridCols<'grid>;

    fn next(&mut self) -> Option<Self::Item> {
        let row_start = self.position * self.grid.cols;
        if row_start < self.grid.data.len() {
            console_log(&format!("Row {}", self.position));
            self.position += 1;
            Some(GridCols{
                grid: self.grid, 
                position: row_start, 
                start: row_start 
            })
        }
        else {
            None
        }
    }
}


/// Color pallete
#[component]
pub fn ColorPallete(
    /// Grid with colors
    colors: Grid,
    /// Signal deciding wherever we show color names
    #[prop(into)]
    show_color_name: Signal<bool>,
) -> impl IntoView {
    view! {
        <div class="leptos-forge-colorpallete scrollbox h-full min-h-[300px] align-top">
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
                        <div class="leptos-forge-colorpallete-row relative h-[100px] align-top">
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

/// Different kinds of color codes
#[derive(Debug, Clone)]
pub enum ColorCode {
    /// Color described as css class 
    /// 
    /// For example tailwind class `"bg-blue-800"`
    CssClass{
        /// Class name
        cls: String,
        /// Color name
        name: String,
    },
    /// Color to be passed into `background-color:` css property
    Color(String),
    /// Lack of color
    /// 
    /// Sometimes some of the colors should not be present in the palette
    /// this is a way to leave the place empty
    Skip,
}

impl ColorCode{
    /// Create a css color code
    pub fn css<S1, S2>(cls: S1, name: S2) -> Self 
    where
        S1: ToString,
        S2: ToString,
    {
        ColorCode::CssClass{ 
            cls: cls.to_string(), 
            name: name.to_string() 
        }
    }
}

/// Shows single color in the color pallete
#[component]
fn Color(
    /// Show color name
    #[prop(into)]
    show_color_name: Signal<bool>,
    /// Color code to be shown
    color: ColorCode,
) -> impl IntoView {
    use ColorCode::*;

    let basic_css = "leptos-forge-colorpallete-color inline-block relative w-[200px] h-[100px] align-top";

    match color {
        CssClass{cls, name} => {
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
        <div class="leptos-forge-colorpallete-color-name text-center text-white forge-text-outline-forgegray-950 forge-text-outline-[0.06em] text-2xl font-bold align-top">
            { name }
        </div>
    }
}