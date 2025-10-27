//! Library with helpers to write a build scripts
//! 
//! | Module | Description |
//! |:-------|:------------|
//! | [console] | Contains the code related to printing messages during the `cargo build`. Has support for multiline text and message tagging |
//! | [resources] | Reliable static resource management across dependencies |
//! | [tailwind] | Reliable tailwind integration for cargo projects |
//! 
//! Detailed documentation and usage can be found at the module level

#![doc(test(attr(deny(unused))))]
#![doc(test(attr(deny(dead_code))))]
#![deny(missing_docs)]
#![deny(clippy::empty_docs)]
#![deny(clippy::missing_docs_in_private_items)]
#![deny(clippy::missing_safety_doc)]
#![deny(clippy::missing_errors_doc)]
#![deny(clippy::missing_fields_in_debug)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![doc(test(attr(deny(unused))))]

pub mod console;
pub mod tailwind;
pub mod resources;
