//! Site for `leptos_forge`
//! 
//! The site is both the website with documentation about the `leptos_forge` and 
//! the example at the same time.
//! 
//! We try to keep it clean, documented and easy to read.

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

mod stories;

use leptos::prelude::*;


use forge::App;
use log::Level;
use stories::ROUTES;

/// Entrypoint of the application
pub fn main() {
    _ = console_log::init_with_level(Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| view!{
        <App routes=ROUTES logo="/resources/leptos_forge/logo/logo.svg" />
    });
}