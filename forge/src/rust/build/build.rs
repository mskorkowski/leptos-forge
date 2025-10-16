//! Build script for the `leptos_forge` crate.
//! 
//! # Requirements
//! 
//! To run a build you must have a tailwindcss 4.* installed
//! 
//! # Build process
//! 
//! 1. Monitor the changes in the `src/css/main.css` file and rebuild the project when there are changes

use std::thread::sleep;
use std::time::Duration;
use std::env::current_dir;
use std::process::Command;
use cargo_metadata::camino::Utf8PathBuf;
use cargo_resources::collate_resources;
use build_print::println;
use build_print::error;
use build_print::info;
use build_print::warn;
use chrono::DateTime;
use chrono::Local;
use chrono::Utc;
use std::path::Path;
use std::fs::metadata;
use std::fs::OpenOptions;
use std::io::prelude::*;

/// Description in case of tailwind execution failure
const TAILWIND_FAILURE: &str = "Build script can't find the tailwindcss cli. Please check your tailwind installation";

/// Tag to prefix the output lines so we know from where it comes from
const CRATE_TAG: &str = "forge";

/// Simple helper function to print the multiline error message to the cargo output
fn error<S: ToString>(s: S) {
    for line in s.to_string().split("\n") {
            error!("[{CRATE_TAG}] {line}");
    }
}

/// Simple helper function to print the multiline warning message to the cargo output
fn warn<S: ToString>(s: S) {
    for line in s.to_string().split("\n") {
        warn!("[{CRATE_TAG}] {line}");
    }
}

/// Simple helper function to print the multiline information message to the cargo output
fn info<S: ToString>(s: S) {
    for line in s.to_string().split("\n") {
            info!("[{CRATE_TAG}] {line}");
    }
    
}

/// Simple helper function to print a normal multiline text to the cargo output
fn println<S: ToString>(s: S) {
    for line in s.to_string().split("\n") {
            println!("[{CRATE_TAG}] {line}");
    }
}

/// Entry point for the build script.
fn main() {
    info("leptos_forge: Running build script!\n");
    info(format!("\tCurrent directory: {}", current_dir().expect("Must have some current directory, no?").display()));
    info(format!("\tOUT_DIR:           {}", std::env::var("OUT_DIR").expect("Cargo doc requires that this variable is set: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts")));
    info(format!("\tCARGO_FEATURE_CLEAN_RESOURCES: {:?}", std::env::var("CARGO_FEATURE_CLEAN_RESOURCES")));
    info("");
    // Print all environment variables.
    // for (key, value) in std::env::vars() {
    //     info(format!("\t\t{key}: {value}"));
    // }
    std::println!("cargo::rerun-if-changed=src/css/main");
    std::println!("cargo::rerun-if-changed=src/resources/logo");

    // We remove the `target/resources` to force the resource resolution by the
    // cargo-resources, no question asked

    if std::env::var("CARGO_FEATURE_CLEAN_RESOURCES").is_ok() {
        let _ = std::fs::remove_dir_all("target/resources");
    }

    // # cargo-resources work around
    //
    // # Current state
    //
    // cargo-resources when collates resources requires all of the resources
    // to be available before collating
    //
    // The problem is when downstream crate needs to provide a resources based on
    // upstream resources.
    //
    // To generate the resource crate needs to collate the resources. To collate
    // the resource the generated resource must be present... And we have a loop
    //
    // # Workaround
    //
    // Create an empty file (if local resources are not present) and always 
    // collate twice (first for the seeding) and second to update the just generated
    // resources provided by this crate
    //
    // Creating an empty file
    let generated_files = vec![
        "target/resources/leptos_forge/main.css"
    ];

    for file in generated_files {
        let path = Path::new(file);
        if !path.exists() {
            let Ok(mut f) = OpenOptions::new().
                create_new(true).
                write(true).
                open(file) else {
                    warn(format!("File '{file}' already exists"));
                    continue;
                };

            let _ = writeln!(f, "  ");
            let _ = f.sync_all();
        }

        let metadata = metadata(path).expect("File should exist");
        let modified = metadata.modified().expect("Update should exist. Maybe?");
        let daytime: DateTime<Utc> = modified.into();
        let daytime: DateTime<Local> = daytime.with_timezone(&Local);
        info(format!("\n\tFile: {} was modified at: {:?}", path.display(), daytime));
    }

    let cwd = current_dir().unwrap();
    let manifest_file = Utf8PathBuf::from_path_buf(cwd).unwrap().join("Cargo.toml");
    let mut cnt = 0;

    // Collate resources from the crate's dependencies so we can generate our 
    // own stuff
    //
    // We run it in the loop because `build script` could be run before all of
    // the resources are available. 
    //
    // > Cargo runs the build script just before a build, but it's not expected
    // > for Cargo build script to
    // > 
    // > 1. Generate resources which would be required downstream
    // > 2. Depend on upstream resources built as described in point 1.
    while let Err(e) = collate_resources(&manifest_file) {
        if cnt > 100 {
            error("It was not possible to bundle the resources");
            return;
        }
        
        warn("Failed to bundle the crate");
        println(format!("\n\ncargo-resources returned an error:\n\n{e}\n\n"));
        sleep(Duration::from_millis(100));
        warn("Retrying...");
        cnt += 1;
    }

    let output = Command::new("tailwindcss")
        // .env("DEBUG", "*")
        .args(vec![
            "--input", "src/css/main/main.css",
            "--output", "target/resources/leptos_forge/main.css",
            "--optimize",
            "--verbose"
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
        info("Tailwind run successfully");
        info("");
        info("--------[ STDOUT ]------------------------");
        info("");
        info(String::from_utf8_lossy(&output.stdout));
        info("");
        info("--------[ STDERR ]------------------------");
        info("");
        info(String::from_utf8_lossy(&output.stderr));
    }

    let generated_files = vec![
        "target/resources/leptos_forge/main.css",
        "target/resources/leptos_forge_ui_components/common.css",
        "target/resources/leptos_forge_ui_components/main.css",
        "target/resources/leptos_forge_ui_components/markdown.css",
        "target/resources/leptos_forge_ui_components/typography.css",
    ];

    for file in generated_files {
        let path = Path::new(file);
        let metadata = metadata(path).expect("File should exist");
        let modified = metadata.modified().expect("Update should exist. Maybe?");
        let daytime: DateTime<Utc> = modified.into();
        let daytime: DateTime<Local> = daytime.with_timezone(&Local);
        info(format!("\tFile: {} was modified at: {:?}", path.display(), daytime));
    }

    // Rerun collate resource so we provide a proper thing downstream
    while let Err(e) = collate_resources(&manifest_file) {
        if cnt > 100 {
            error("It was not possible to bundle the resources");
            return;
        }
        
        warn("Failed to bundle the crate");
        println(format!("\n\ncargo-resources returned an error:\n\n{e}\n\n"));
        sleep(Duration::from_millis(100));
        warn("Retrying...");
        cnt += 1;
    }

}