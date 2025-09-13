//! Storybook for components code

#![doc(test(attr(deny(unused))))]
#![doc(test(attr(deny(dead_code))))]
#![allow(dead_code)]

#![deny(missing_docs)]
#![deny(clippy::empty_docs)]
#![deny(clippy::missing_docs_in_private_items)]
#![deny(clippy::missing_docs_in_private_items)]
#![deny(clippy::missing_safety_doc)]
#![deny(clippy::missing_errors_doc)]
#![deny(clippy::missing_fields_in_debug)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![doc(test(attr(deny(unused))))]

pub mod app;
pub mod navigation;
pub mod section;
pub mod story;
pub mod views;

pub use app::*;
pub use navigation::*;
pub use section::*;
pub use story::*;
