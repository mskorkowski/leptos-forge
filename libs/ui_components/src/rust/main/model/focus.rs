//! Structures related to handling the focus management of the widgets
//! 
//! Sometimes you can't get without it
//! 

/// Defines possible states of focus of the widget in the reactive system
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Focus {
    /// Value signals that widget should steal the focus for itself
    /// 
    /// After handling the `Grab` signal widget should inform the rest of the
    /// system that it's either [`In`][Focus::In] since focus was stolen for
    /// the widget or it should give up with stealing using [`Out`][Focus::Out].
    Grab,
    /// Value signals that widget is in focus
    /// 
    /// This signal should be sent in cases:
    /// 
    /// 1. Widget got in focus because of user interaction with the system
    /// 2. Widget received [`Grab`][Focus::Grab]
    In,
    /// Value signal that widget should blur itself
    /// 
    /// After handling `Blur` signal widget should inform the rest of the
    /// system that it's [`Out`][Focus::Out] of focus
    Blur,
    /// Value signals that widget is out of focus
    /// 
    /// This signal should be sent in cases:
    /// 
    /// 1. Widget get blurred by user interaction
    /// 2. Widget received [`Blur`][Focus::Blur]
    Out,
}