//! Enumerable trait
//! 
//! This trait should be implemented for types
//! 

/// Trait provides iterator over type (like enum) instead of enumeration over 
/// values.
/// 
/// If you would like to iterate over value use `Iterator`/`IntoIterator` trait
/// from standard library
/// 
/// # Example
/// 
/// ```rust
/// 
/// #use leptos_forge_utils:enumerable::Enumerable;
/// 
/// enum MenuItems {
///     Home,
///     Tasks,
///     Emails,
/// }
/// 
/// impl Enumerable for MenuItems {
/// 
///     fn iter() -> impl Iterator<Item = Self> {
///         use MenuItem::*;
///         vec![
///             Home,
///             Tasks,
///             Emails,
///         ].into_iter()
///     }
/// 
/// }
/// 
/// let iter = MenuItems::iter();
/// 
/// assert(MenuItem::Home, iter.next());
/// assert(MenuItem::Tasks, iter.next());
/// assert(MenuItem::Emails, iter.next());
/// 
/// ```
/// 
/// # Features
/// 
/// ## strum integration
/// 
/// If you already use the [`strum`][https://crates.io/crates/strum] crate you can
/// enable the `strum` feature which will implement `Enumerable` for any trait
/// which has `EnumIter` derived
pub trait Enumerable {
    /// Return iterator over type
    fn iter() -> impl Iterator<Item = Self>;
}

#[cfg(feature = "strum")]
mod strum_impl {
    //! Provides implementation of the [`Enumerable`] if you use the 
    //! [`strum`][https://crates.io/crates/strum] crate 
    use super::*;
    use strum::IntoEnumIterator;

    impl<T: IntoEnumIterator> Enumerable for T {
        #[inline]
        fn iter() -> impl Iterator<Item = Self> {
            <Self as IntoEnumIterator>::iter()
        }
    }
}