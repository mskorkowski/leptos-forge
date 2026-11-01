//! Library with various utils which doesn't belong anywhere else yet

#![doc(test(attr(deny(unused))))]
#![doc(test(attr(deny(dead_code))))]

extern crate leptos;

pub mod breakpoints;
pub mod css;
pub mod element;
pub mod signal;
pub mod stores;

/// Publicly exports the basic functionality of the `leptos_forge_utils_leptos`
pub mod prelude {
    use super::*;

    pub use breakpoints::*;
    pub use css::*;
    pub use element::*;
    pub use signal::*;
    pub use stores::stored_ref::*;
}