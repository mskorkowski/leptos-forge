//! Leptos forge site build script
//! 
//! The main goal of this script is to pull all upstream resources so
//! they can be used in the site build process.


use std::env::current_dir;
use cargo_metadata::camino::Utf8PathBuf;
use cargo_resources::collate_resources;

fn main() {
    println!("cargo::rerun-if-changed=src/css/main");
    println!("cargo::rerun-if-changed=src/js/loader");
    println!("cargo::rerun-if-changed=src/resources");

    let cwd = current_dir().unwrap();
    let manifest_file = Utf8PathBuf::from_path_buf(cwd).unwrap().join("Cargo.toml");

    // Collate resources from the crate's dependencies.
    collate_resources(&manifest_file).expect("There was an error during building of the resources");
}