//! Build script for the `leptos_forge` crate.
//!
//! # Requirements
//!
//! To run a build you must have a tailwindcss 4.* installed
//!
//! # Build process
//!
//! 1. Monitor the changes in the `src/css/main.css` file and rebuild the project when there are changes

// use build_print::error;
use build_print::info;
// use build_print::println;
// use build_print::warn;
// use cargo_metadata::camino::Utf8PathBuf;
// use cargo_resources::collate_resources_with_crate_filter;
// use cargo_resources::reporting::ReportingTrait;
// use chrono::DateTime;
// use chrono::Local;
// use chrono::Utc;
use std::env::current_dir;
// use std::fs::metadata;
// use std::path::Path;
// use std::process::Command;
// use std::thread::sleep;
// use std::time::Duration;
// use serde_json::Error;

/// Tag to prefix the output lines so we know from where it comes from
const CRATE_TAG: &str = "forge";

// /// Name of the crate
// const CRATE: &str = "leptos_forge";

// /// Simple helper function to print the multiline error message to the cargo output
// fn error<S1: ToString, S2: ToString>(stage: S1, s: S2) {
//     let stage = stage.to_string();
//     for line in s.to_string().split("\n") {
//         error!("[{CRATE_TAG}][{stage}] {line}");
//     }
// }

// /// Simple helper function to print the multiline warning message to the cargo output
// fn warn<S1: ToString, S2: ToString>(stage: S1, s: S2) {
//     let stage = stage.to_string();
//     for line in s.to_string().split("\n") {
//         warn!("[{CRATE_TAG}][{stage}] {line}");
//     }
// }

/// Simple helper function to print the multiline information message to the cargo output
fn info<S1: ToString, S2: ToString>(stage: S1, s: S2) {
    let stage = stage.to_string();
    for line in s.to_string().split("\n") {
        info!("[{CRATE_TAG}][{stage}] {line}");
    }
}

// /// Simple helper function to print a normal multiline text to the cargo output
// fn println<S1: ToString, S2: ToString>(stage: S1, s: S2) {
//     let stage = stage.to_string();
//     for line in s.to_string().split("\n") {
//         println!("out:  [{CRATE_TAG}][{stage}] {line}");
//     }
// }

// /// Prints the output in the cargo compatible way
// struct ResourcesReporter{
//     /// Which step we use to run the resource collation
//     stage: &'static str,
// }

// impl ResourcesReporter {
//     /// Create new instance of the [ResourceReporter]
//     pub fn new(stage: &'static str) -> Self {
//         ResourcesReporter { stage }
//     }
// }


// impl ReportingTrait for ResourcesReporter{
//     fn report_duplicate_resource(&self, resolved_name: &str, replaced: &Utf8PathBuf, with: &Utf8PathBuf) {
//         error(self.stage, format!(
// r#"Duplicate resource
//     Resource: {resolved_name}
//         {replaced} -> {with}
// "#
//         ));
//     }

//     fn report_malformed_resource_declaration(&self, package_name: &str, err: &Error) {
//         error(self.stage, format!(
// r#"Malformed resource declaration in package {package_name}

// {err}
// "#
//         ))
//     }

//     fn report_malformed_resources_section(&self) {
        
//     }

//     fn report_missing_resource(&self, resource_name: &str) {
//         error(self.stage, format!(
// r#"Missing resource
//     Missing: {resource_name}
// "#
//         ));
//     }

//     fn report_no_resources_found(&self) {
//         error(self.stage, r#"No resources"#);
//     }

//     fn report_resource_collection(&self, already_existed: bool, new_sha: &str, output_path: &Utf8PathBuf) {
//         if already_existed {
//             info(self.stage, format!(
//                 "[ KEEP ] {output_path}"
//             ));
//         }
//         else {
//             warn(self.stage, format!(
//                 "[UPDATE] {output_path}"
//             ));
//         }
//         info(self.stage, format!(
//             "\t{new_sha}"
//         ));
//     }
// }

// /// Description in case of tailwind execution failure
// const TAILWIND_FAILURE: &str =
//     "Build script can't find the tailwindcss cli. Please check your tailwind installation";

/// Entry point for the build script.
fn main() {
    let stage = "start";
    info(stage, "leptos_forge: Running build script!\n");
    info(stage,format!(
        "\tCurrent directory: {}",
        current_dir()
            .expect("Must have some current directory, no?")
            .display()
    ));
    info(stage,format!("\tOUT_DIR:           {}", std::env::var("OUT_DIR").expect("Cargo doc requires that this variable is set: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts")));
    info(stage,format!(
        "\tCARGO_FEATURE_CLEAN_RESOURCES: {:?}",
        std::env::var("CARGO_FEATURE_CLEAN_RESOURCES")
    ));
    info(stage,"");
    // Print all environment variables.
    // for (key, value) in std::env::vars() {
    //     info(format!("\t\t{key}: {value}"));
    // }
    std::println!("cargo::rerun-if-changed=src/css/main");
    std::println!("cargo::rerun-if-changed=src/resources/logo");

    // We remove the `target/resources` to force the resource resolution by the
    // cargo-resources, no question asked

    // if std::env::var("CARGO_FEATURE_CLEAN_RESOURCES").is_ok() {
    //     let _ = std::fs::remove_dir_all("target/resources");
    // }

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
    // let generated_files = vec!["target/resources/leptos_forge/main.css"];

    // for file in generated_files {
    //     let path = Path::new(file);
    //     if !path.exists() {
    //         let Ok(mut f) = OpenOptions::new().create_new(true).write(true).open(file) else {
    //             warn(format!("File '{file}' already exists"));
    //             continue;
    //         };

    //         let _ = writeln!(f, "  ");
    //         let _ = f.sync_all();
    //     }

    //     let metadata = metadata(path).expect("File should exist");
    //     let modified = metadata.modified().expect("Update should exist. Maybe?");
    //     let daytime: DateTime<Utc> = modified.into();
    //     let daytime: DateTime<Local> = daytime.with_timezone(&Local);
    //     info(format!(
    //         "\n\tFile: {} was modified at: {:?}",
    //         path.display(),
    //         daytime
    //     ));
    // }

    // let cwd = current_dir().unwrap();
    // let manifest_file = Utf8PathBuf::from_path_buf(cwd).unwrap().join("Cargo.toml");
    // let mut cnt = 0;


    // let stage = "upstream";
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
    // while let Err(e) = collate_resources_with_crate_filter(
    //     &manifest_file, 
    //     &ResourcesReporter::new(stage),
    //     |pkg| {
    //         pkg.name != CRATE
    //     }
    // ) {
    //     if cnt > 100 {
    //         error(stage, "It was not possible to bundle the resources");
    //         return;
    //     }

    //     warn(stage, "Failed to bundle the crate");
    //     println(stage, format!("\n\ncargo-resources returned an error:\n\n{e}\n\n"));
    //     sleep(Duration::from_millis(100));
    //     warn(stage, "Retrying...");
    //     cnt += 1;
    // }

    // let stage = "tailwind";

    // let output = Command::new("tailwindcss")
    //     // .env("DEBUG", "*")
    //     .args(vec![
    //         "--input",
    //         "src/css/main/main.css",
    //         "--output",
    //         "target/resources/leptos_forge/main.css",
    //         "--optimize",
    //         "--verbose",
    //     ])
    //     .output()
    //     .expect(TAILWIND_FAILURE);

    // if !output.status.success() {
    //     error(stage, format!("Tailwind Stopped with an {}", output.status));
    //     error(stage, "");
    //     error(stage, "--------[ STDOUT ]------------------------");
    //     error(stage, "");
    //     println(stage, String::from_utf8_lossy(&output.stdout));
    //     error(stage, "");
    //     error(stage, "--------[ STDERR ]------------------------");
    //     error(stage, "");
    //     error(stage, String::from_utf8_lossy(&output.stderr));
    // } else {
    //     info(stage, "Tailwind run successfully");
    //     info(stage, "");
    //     info(stage, "--------[ STDOUT ]------------------------");
    //     info(stage, "");
    //     info(stage, String::from_utf8_lossy(&output.stdout));
    //     info(stage, "");
    //     info(stage, "--------[ STDERR ]------------------------");
    //     info(stage, "");
    //     info(stage, String::from_utf8_lossy(&output.stderr));
    // }

    // let stage = "check";

    // let generated_files = vec![
    //     "target/resources/leptos_forge/main.css",
    //     "target/resources/leptos_forge_ui_components/common.css",
    //     "target/resources/leptos_forge_ui_components/main.css",
    //     "target/resources/leptos_forge_ui_components/markdown.css",
    //     "target/resources/leptos_forge_ui_components/typography.css",
    // ];

    // for file in generated_files {
    //     let path = Path::new(file);
    //     let metadata = metadata(path).expect("File should exist");
    //     let modified = metadata.modified().expect("Update should exist. Maybe?");
    //     let daytime: DateTime<Utc> = modified.into();
    //     let daytime: DateTime<Local> = daytime.with_timezone(&Local);
    //     info(stage, format!(
    //         "\tFile: {} was modified at: {:?}",
    //         path.display(),
    //         daytime
    //     ));
    // }

    // let stage = "self collate";

    // // Rerun collate resource so we provide a proper thing downstream
    // while let Err(e) = collate_resources_with_crate_filter(
    //     &manifest_file, 
    //     &ResourcesReporter::new("Self"),
    //     |pkg| {
    //         pkg.name == CRATE
    //     }
    // ) {
    //     if cnt > 100 {
    //         error(stage, "It was not possible to bundle the resources");
    //         return;
    //     }

    //     warn(stage, "Failed to bundle the crate");
    //     println(stage, format!("\n\ncargo-resources returned an error:\n\n{e}\n\n"));
    //     sleep(Duration::from_millis(100));
    //     warn(stage, "Retrying...");
    //     cnt += 1;
    // }
}
