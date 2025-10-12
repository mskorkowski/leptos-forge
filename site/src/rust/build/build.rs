//! Leptos forge site build script
//! 
//! The main goal of this script is to pull all upstream resources so
//! they can be used in the site build process.


use std::env::current_dir;
use std::process::Command;
use cargo_metadata::camino::Utf8PathBuf;
use cargo_resources::collate_resources;
use build_print::{println, error, info};

/// Description in case of tailwind execution failure
const TAILWIND_FAILURE: &str = "Build script can't find the tailwindcss cli. Please check your tailwind installation";

/// Simple helper function to print the multiline string to the cargo output
fn error<S: ToString>(s: S) {
    for line in s.to_string().split("\n") {
            error!("[site] {line}");
    }
}

/// Simple helper function to print the multiline string to the cargo output
fn info<S: ToString>(s: S) {
    for line in s.to_string().split("\n") {
            info!("[site] {line}") ;
    }
    
}

/// Simple helper function to print a normal multiline test to the cargo output
fn println<S: ToString>(s: S) {
    for line in s.to_string().split("\n") {
            println!("[site] {line}");
    }
}

fn main() {
    std::println!("cargo::rerun-if-changed=src/css/main");
    std::println!("cargo::rerun-if-changed=src/js/loader");
    std::println!("cargo::rerun-if-changed=src/resources");

    let cwd = current_dir().unwrap();
    let manifest_file = Utf8PathBuf::from_path_buf(cwd).unwrap().join("Cargo.toml");

    // Collate resources from the crate's dependencies.
    collate_resources(&manifest_file).expect("[site] There was an error during bundling of the resources");

    let output = Command::new("tailwindcss")
        .args(vec![
            "--input", "src/css/main/main.css",
            "--output", "target/resources/leptos_forge_site/main.css",
            "--optimize"
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
    else {
        info("[SITE] Tailwind run successfully");
    }
}