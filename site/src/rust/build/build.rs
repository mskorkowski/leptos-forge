//! Leptos forge site build script
//!
//! The main goal of this script is to pull all upstream resources so
//! they can be used in the site build process.

use build_script::console::Console;
use build_script::console::ConsoleConfiguration;
use build_script::tailwind::Tailwind;
use cargo_metadata::CargoOpt;
use cargo_metadata::Metadata;
use cargo_metadata::MetadataCommand;
use cargo_metadata::camino::Utf8PathBuf;
use cargo_resources::collate_resources_with_reporting;
use cargo_resources::reporting::ReportingTrait;
use indoc::formatdoc;
use std::env::current_dir;
use serde_json::Error;

/// New type for Console to handle the `cargo_resource` reporting
struct Reporter<'this>(Console<'this>);

impl<'this> ReportingTrait for Reporter<'this> {
    // overridden_package, overriding_package
    fn report_duplicate_resource(&self, resolved_name: &str, replaced: &Utf8PathBuf, with: &Utf8PathBuf) {
        self.0.warn(&formatdoc!(r#"
            Duplicate resource for {resolved_name}

                Original: {replaced}
                New:      {with}
            "#
        ));
    }

    // package missing the resource
    // 
    fn report_missing_resource(&self, resolved_name: &str) {
        self.0.warn(&formatdoc!(r#"
            Required resource not found

                Missing resource: {resolved_name}
            "#
        ));
    }   

    // value
    fn report_malformed_resource_declaration(&self, package_name: &str, err: &Error) {
        panic!(
            "{}",
            formatdoc!(r#"
                Resource declaration in {package_name} is malformed

                    Cause: {err}
                "#
            )
        )
    }

    // package
    // value
    fn report_malformed_resources_section(&self) {
        panic!(
            "{}",
            formatdoc!(r#"
                Malformed [[package.metadata.cargo_resources.provides]] section

                  - [[package.metadata.cargo_resources.provides]] should be a list of
                    resource definitions
                "#
            )
        )
    }

    fn report_no_resources_found(&self) {
        self.0.warn(&formatdoc!(r#"
            No resources to collate
            "#
        ));
    }

    fn report_resource_collection(&self, already_existed: bool, new_sha: &str, output_path: &Utf8PathBuf) {
        let exists = if already_existed {
            "[present]"
        }
        else {
            "[  new  ]"
        };
        self.0.info(&format!("{exists} {new_sha} {output_path}"))
    }
}

fn main() {
    let console_configuration = ConsoleConfiguration::default();
    let console = Console::new("site", &console_configuration);
    
    console.info(&"Running build script!\n");
    console.info(&format!(
        "\tCurrent directory: {}",
        current_dir()
            .expect("Must have some current directory, no?")
            .display()
    ));
    console.info(&format!("\tOUT_DIR:           {}", std::env::var("OUT_DIR").expect("Cargo doc requires that this variable is set: https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts")));
    std::println!("cargo::rerun-if-changed=src/css/main");
    std::println!("cargo::rerun-if-changed=src/js/loader");
    std::println!("cargo::rerun-if-changed=src/resources");

    // We remove the `target/resources` to force the resource resolution by the
    // cargo-resources, no question asked
    if std::env::var("CARGO_FEATURE_CLEAN_RESOURCES").is_ok() {
        let _ = std::fs::remove_dir_all("target/resources");
    }

    let cwd = current_dir().unwrap();
    let manifest_file = Utf8PathBuf::from_path_buf(cwd).unwrap().join("Cargo.toml");

    {
        let mut metadata_cmd = MetadataCommand::new();
        let metadata: Metadata = metadata_cmd
            .manifest_path(&manifest_file)
            .features(CargoOpt::AllFeatures)
            .exec()
            .unwrap();

        let console = console.stage("tailwind");
        let _output_path = Tailwind::new(metadata, &console, None, true).
            run().
            unwrap();
    }

    {
        let console = console.stage("resources");

        // Collate resources from the crate's dependencies.
        collate_resources_with_reporting(&manifest_file, &Reporter(console))
            .expect("[site] There was an error during bundling of the resources");
    }
}
