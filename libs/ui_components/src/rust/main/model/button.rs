//! Click states for interface

use std::ops::Add;

/// Possible button click events
#[non_exhaustive]
#[derive(Default, Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonClick {
    /// This state can be set to denote that the button is released and no action
    /// should be taken.
    ///
    /// It is useful as the initial stream state.
    #[default]
    Released,
    /// Single click with left mouse button or
    /// keyboard based trigger with <kbd>space</kbd> or <kbd>enter</kbd> key
    LeftClick,
    /// Single click with right mouse button or
    /// <kbd>alt</kbd> with <kbd>space</kbd> or <kbd>enter</kbd> key
    RightClick,
    /// Single click with middle mouse button or
    /// <kbd>alt</kbd> + <kbd>shift</kbd> with <kbd>space</kbd> or <kbd>enter</kbd> key
    MiddleClick,
}

/// Defines possible button groupings
#[derive(Debug, Clone, PartialEq)]
pub enum ButtonGroup {
    /// Only one button at the same time can be selected
    /// 
    /// Equivalent of the radio button group
    SingleSelect(String),
    /// Multiple buttons at the same time can be selected
    /// 
    /// Equivalent of the group of checkboxes
    MultiSelect
}

/// Color schema for buttons
#[derive(Debug, Clone, Copy)]
pub struct ButtonColorSchema {
    /// Css classes for the button describing outer parts of the button
    /// 
    /// - display
    /// - margin
    /// - outline
    /// - ...
    pub outer: &'static str,
    /// Css classes describing inner part of the button
    /// 
    /// - padding
    /// - internal layout
    /// - ...
    pub inner: &'static str,
    /// Border color
    pub border: &'static str,
    /// Text color
    pub color: &'static str,
    /// Background color
    pub background: &'static str,
}

impl IntoButtonColorSchema for ButtonColorSchema {
    fn as_button_color_schema(&self) -> ButtonColorSchema {
        *self
    }
}

/// Allows conversion into button color schema
pub trait IntoButtonColorSchema {
    /// Converts a value into button color schema
    fn as_button_color_schema(&self) -> ButtonColorSchema;
}

/// Various styles in which button can appear
#[derive(Debug, Clone, Copy, Default)]
pub enum Kind {
    /// Primary button
    /// 
    /// It has
    /// 
    /// - background
    /// - border
    /// 
    /// # Usage
    /// 
    /// It should be used for the cases like default choice or call to action
    Primary,
    /// Secondary button
    /// 
    /// It has 
    /// 
    /// - border
    /// 
    /// # Usage
    /// 
    /// It should be used for the secondary choices like "cancel" in the "Save
    /// changes dialog".
    #[default]
    Secondary,
    /// Invisible button
    /// 
    /// It doesn't have any button cues
    /// 
    /// # Usage
    /// 
    /// It should be used in places like menu buttons where you don't need
    /// extra affordance indicators 
    Invisible
}

impl Kind{
    /// Using color schema generates the list of class names for the most basic
    /// kind of the button
    pub(crate) fn into_button_class(self, schema: &ButtonColorSchema, size: &Size) -> String {
        use Kind::*;

        match self {
            Primary => format!("{} {} {} {} {} {}", schema.outer, schema.inner, schema.background, schema.border, schema.color, size.into_css_class()),
            Secondary => format!("{} {} {} {} {}", schema.outer, schema.inner, schema.border, schema.color, size.into_css_class()),
            Invisible => format!("{} {} {} {}", schema.outer, schema.inner, schema.color, size.into_css_class()),
        }
    }

    // pub(crate) fn into_outer_box_class(&self, schema: &ButtonColorSchema, size: &Size) -> String {
    //     match self {
    //         Primary => format!("{} {} {} {} {}", schema.outer, schema.background, schema.border, schema.color, size.into_css_class()),
    //         Secondary => format!("{} {} {} {}", schema.outer, schema.border, schema.color, size.into_css_class()),
    //         Invisible => format!("{} {} {}", schema.outer, schema.color, size.into_css_class()),
    //     }
    // }

    /// Creates a style of the button
    /// 
    /// # Returns
    /// 
    /// Returns the [Style] for [Size::Normal]
    pub fn color<Schema: IntoButtonColorSchema>(self, schema: Schema) -> Style {
        Style{
            kind: self,
            schema: schema.as_button_color_schema(),
            size: Size::default()
        }
    }
}

/// Size of the button
#[derive(Debug, Clone, Copy, Default)]
pub enum Size {
    /// Big button
    Big,
    /// Normal sized button
    #[default]
    Normal,
    /// Small sized button
    Small,
}

impl Size {
    /// Returns the css classes describing the size of the button
    fn into_css_class(self) -> &'static str {
        use Size::*;

        match self {
            Big => "px-3 py-2",
            Normal => "px-2 py-1.5",
            Small => "px-1"
        }
    }

}

/// Button style
/// 
/// # Creating an instance
/// 
/// To create an instance use [Kind::color] method
/// 
/// ```
/// let schema: ButtonColorSchema = ButtonColorSchema{
///   inner: "",
///   outer: "",
///   border: "border-solid border-gray-200",
///   color: "gray-50",
///   background: "bg-gray-800"
/// }
/// 
/// let kind = Kind::Primary.color(schema);
/// 
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Style {
    /// Kind of the button
    pub kind: Kind,
    /// Color schema
    pub schema: ButtonColorSchema,
    /// Size of the button
    pub size: Size,
}

impl From<ButtonColorSchema> for Style {
    /// Returns a style with given color schema and where [Kind] and [Size] are
    /// set to default values
    fn from(schema: ButtonColorSchema) -> Self {
        Style{ 
            kind: Kind::default(), 
            schema, 
            size: Size::default(), 
        }
    }
}

impl Add<Size> for Style {
    type Output = Style;

    fn add(self, rhs: Size) -> Self::Output {
        Style{
            size: rhs,
            ..self
        }
    }
}

impl Add<Kind> for Style {
    type Output = Style;

    fn add(self, rhs: Kind) -> Self::Output {
        Style{
            kind: rhs,
            ..self
        }
    }
}

impl Add<ButtonColorSchema> for Style {
    type Output = Style;

    fn add(self, rhs: ButtonColorSchema) -> Self::Output {
        Style{
            schema: rhs,
            ..self
        }
    }
}