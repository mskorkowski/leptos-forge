//! Model datastructures for [ColorPallette] widget

/// Different kinds of color codes
#[derive(Debug, Clone, PartialEq)]
pub enum ColorCode {
    /// Color described as css class 
    /// 
    /// For example tailwind class `"bg-blue-800"`
    CssClass{
        /// Class name
        cls: &'static str,
        /// Color name
        name: &'static str,
        /// variable
        variable: &'static str,
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
    pub const fn css (
        cls: &'static str, 
        name: &'static str, 
        variable: &'static str
    ) -> Self {
        // ColorCode::CssClass{ 
        //     cls: cls.to_string(), 
        //     name: name.to_string(),
        //     variable: variable.to_string(),
        // }
        ColorCode::CssClass{ 
            cls, 
            name,
            variable,
        }
    }

    /// Get the class name
    /// 
    /// If it's not a css class name it returns an empty string
    pub fn background_class_name_or_empty(&self) -> &str {
        use ColorCode::*;

        match self {
            CssClass { cls, .. } => //cls.as_str(),
                cls, 
            _ => ""

        }
    }

    /// Get the color variable name
    /// 
    /// If it's not a css class name it returns an empty string
    pub fn variable_or_empty(&self) -> &str {
        use ColorCode::*;

        match self {
            CssClass { variable, .. } => //variable.as_str(),
                                                        variable,
            _ => ""
        }
    }
}

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
            let current= self.grid.data.get(self.position);
            self.position += 1;
            current // we know it's some, by the if statement
        }
        else if self.position - self.start < self.grid.cols {
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

