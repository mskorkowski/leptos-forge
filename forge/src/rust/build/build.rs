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
use build_print::{println, error, info};
/// Description in case of tailwind execution failure
const TAILWIND_FAILURE: &str = "Build script can't find the tailwindcss cli. Please check your tailwind installation";

/// Simple helper function to print the multiline string to the cargo output
fn error<S: ToString>(s: S) {
    for line in s.to_string().split("\n") {
            error!("{line}");
    }
}

/// Simple helper function to print the multiline string to the cargo output
fn info<S: ToString>(s: S) {
    for line in s.to_string().split("\n") {
            info!("{line}");
    }
    
}

/// Simple helper function to print a normal multiline test to the cargo output
fn println<S: ToString>(s: S) {
    for line in s.to_string().split("\n") {
            println!("{line}");
    }
}

/// Entry point for the build script.
fn main() {
    info("leptos_forge: Running build script!");
    std::println!("cargo::rerun-if-changed=src/css/main");
    std::println!("cargo::rerun-if-changed=src/resources/logo");


    let output = Command::new("tailwindcss")
        .args(vec![
            "--input", "src/css/main/main.css",
            "--output", "src/resources/generated/leptos-forge.css"
        ])
        .output()
        .expect(TAILWIND_FAILURE);

    if !output.status.success() {
        error(format!("Tailwind Stopped with an {}", output.status));
        error("");
        error("--------[ STDOUT ]------------------------");
        error("");
        println(String::from_utf8_lossy(&output.stdout));
        error("");
        error("--------[ STDERR ]------------------------");
        error("");
        error(String::from_utf8_lossy(&output.stderr));

    }


}