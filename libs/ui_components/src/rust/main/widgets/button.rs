//! Various kinds of button widgets to be used

use std::ops::Add;

use leptos::prelude::*;

pub use super::super::primitives::button::ButtonClick;
pub use super::super::primitives::button::Button as PrimitiveButton;

/// Color schema for buttons
#[derive(Debug, Clone, Copy)]
pub struct ColorSchema {
    /// Border color
    pub border: &'static str,
    /// Text color
    pub color: &'static str,
    /// Background color
    pub background: &'static str,
}

impl IntoButtonColorSchema for ColorSchema {
    fn as_button_color_schema(&self) -> ColorSchema {
        *self
    }
}

/// Allows conversion into button color schema
pub trait IntoButtonColorSchema {
    /// Converts a value into button color schema
    fn as_button_color_schema(&self) -> ColorSchema;
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
    /// Using color schema generates the list of class names
    fn into_css_class(self, schema: &ColorSchema, size: &Size) -> String {
        use Kind::*;

        match self {
            Primary => format!("{} {} {} {}", schema.background, schema.border, schema.color, size.into_css_class()),
            Secondary => format!("{} {} {}", schema.border, schema.color, size.into_css_class()),
            Invisible => format!("{} {}", schema.color, size.into_css_class()),
        }
    }

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
            Normal => "px-2 py-1",
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
/// let schema: ColorSchema = ColorSchema{
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
    kind: Kind,
    /// Color schema
    schema: ColorSchema,
    /// Size of the button
    size: Size,
}

impl From<ColorSchema> for Style {
    /// Returns a style with given color schema and where [Kind] and [Size] are
    /// set to default values
    fn from(schema: ColorSchema) -> Self {
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

impl Add<ColorSchema> for Style {
    type Output = Style;

    fn add(self, rhs: ColorSchema) -> Self::Output {
        Style{
            schema: rhs,
            ..self
        }
    }
}


/// Button widget
#[component]
pub fn Button<
    Id: ToString,
>(
    /// Id of the button
    id: Id,
    /// Signal which triggers on clicks
    /// Signal triggered when the button is clicked
    #[prop(into)]
    click: SignalSetter<ButtonClick>,
    /// kind of the button
    #[prop(into, name="style")]
    Style{kind, schema, size}: Style,
    /// Content of the button
    #[prop(optional, default=Box::new(|| view!{"Press me!"}.into_any()))]
    children: Children,
) -> impl IntoView {
    
    let class = kind.into_css_class(&schema, &size);


    view!{
        <PrimitiveButton<Id, String>
            id
            class
            click
            children
        />
    }
}