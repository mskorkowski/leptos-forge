//! Build script for leptos_forge_ui_components

use build_print::error;
use build_print::info;
use build_print::println;
use std::process::Command;

/// Description in case of tailwind execution failure
const TAILWIND_FAILURE: &str =
    "Build script can't find the tailwindcss cli. Please check your tailwind installation";

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

fn main() {
    std::println!("cargo::rerun-if-changed=src/css/main");

    let output = Command::new("tailwindcss")
        .args(vec![
            "--input",
            "src/css/main/main.css",
            "--output",
            "target/resources/leptos_forge_ui_components/main.css",
            "--optimize",
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
    } else {
        info("[UI_COMPONENTS] Tailwind run successfully");
    }
}
