//! Leptos forge site build script
//!
//! The main goal of this script is to pull all upstream resources so
//! they can be used in the site build process.

use build_script::console::Console;
use build_script::console::ConsoleConfiguration;
use build_script::resources::Resources;
use build_script::tailwind::Tailwind;
use cargo_metadata::CargoOpt;
use cargo_metadata::Metadata;
use cargo_metadata::MetadataCommand;
use cargo_metadata::camino::Utf8PathBuf;
use std::env::current_dir;


fn main() {
    let console_configuration = ConsoleConfiguration::default();
    let console = Console::new("site", &console_configuration);
    
    std::println!("cargo::rerun-if-changed=src/css/main");
    std::println!("cargo::rerun-if-changed=src/js/loader");
    std::println!("cargo::rerun-if-changed=src/resources");

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
        let resources = Resources::all(&console, true);
        resources.run().unwrap();
    }
}
