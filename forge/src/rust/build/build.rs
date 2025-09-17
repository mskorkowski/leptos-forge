//! Build script for the `leptos_forge` crate.
//! 
//! # Requirements
//! 
//! To run a build you must have a tailwindcss 4.* installed
//! 
//! # Build process
//! 
//! 1. Monitor the changes in the `src/css/main.css` file and rebuild the project when there are changes

use std::process::Command;

/// Description in case of tailwind execution failure
const TAILWIND_FAILURE: &str = r############"Failed to build css for leptos-forge.

1. Please check your tailwind installation
"############;

/// Entry point for the build script.
fn main() {
    println!("cargo::rerun-if-changed=src/css/main");
    println!("cargo::rerun-if-changed=src/resources/logo");

    Command::new("tailwindcss")
        .args(vec![
            "--input", "src/css/main/main.css",
            "--output", "src/resources/generated/leptos-forge.css"
        ])
        .output()
        .expect(TAILWIND_FAILURE);
}