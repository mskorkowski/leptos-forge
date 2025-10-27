//! Reliable resource management for your crate based on [cargo-resources](https://github.com/PeteEvans/cargo-resources)
//! 
//! - Works with crate.io
//! - Has ability to make cargo aware of changes in local dependencies 
//! 
//! # Basic usage
//! 
//! Resource provisioning in the `build_script` is a thin wrapper around the `cargo_resources`
//! itself.
//! 
//! 
//! First step is to setup your crates `Cargo.toml` as described in the [`cargo-resources` documentation](https://github.com/PeteEvans/cargo-resources).
//! 
//! In your build script
//! 
//! ```rust,no_run
//! # #[allow(clippy::needless_doctest_main)]
//! use leptos_forge_build_script::{
//!   console::{
//!     Console,
//!     ConsoleConfiguration,
//!   },
//!   resources::Resources,
//! };
//! 
//! fn main() {
//!     // Setup printing messages to the console
//!     let console_configuration = ConsoleConfiguration::default();
//!     let console = Console::new("crate_name", &console_configuration);
//! 
//!     { 
//!         // All output from the tailwind integration will have a `[tailwind]` tag prepended
//!         let console = console.stage("resources");
//! 
//!         // provide all resources
//!         let resources = Resources::all(
//!             &console, 
//!             // If set to `true` it will generate the `cargo::rerun-if-changed=`
//!             // statements for all resources in local crates (same workspace)
//!             true
//!         );
//!         resources.run().unwrap();
//!     }
//! }
//! ```
//! 
//! > **Note**:
//! >
//! > Setting the second argument of `Resources::all` to `true` makes the `leptos_forge_build_script`
//! > generate [`cargo::rerun-if-changed=`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed) 
//! > statements, which disables the default build script behavior. Depending on 
//! > your other use cases it might require to adjust your build script behavior. 
//! >
//! > Cargo defaults can be brought back by adding
//! >
//! > ```rust
//! > std::println!("cargo::rerun-if-changed=.")
//! > ```
//! 

use std::env::current_dir;

use cargo_metadata::Metadata;
use cargo_metadata::Package;
use cargo_metadata::camino::Utf8PathBuf;
use cargo_resources::collate_resources_with_crate_filter;
use cargo_resources::reporting::ReportingTrait;
use indoc::formatdoc;
use serde_json::Error;

use crate::console::Console;

/// New type for Console to handle the `cargo_resource` reporting
struct Reporter<'this>(&'this Console<'this>);

impl<'this> ReportingTrait for Reporter<'this> {
    fn report_duplicate_resource(&self, resolved_name: &str, replaced: &Utf8PathBuf, with: &Utf8PathBuf) {
        // TODO: ask the cargo_resources for overridden_package, overriding_package, source paths, non-canonical form of target paths
        self.0.warn(&formatdoc!(r#"
            Duplicate resource for {resolved_name}

                Original: {replaced}
                New:      {with}
            "#
        ));
    }

    fn report_missing_resource(&self, resolved_name: &str) {
        // TODO: ask the cargo_resources for package missing the resource
        self.0.warn(&formatdoc!(r#"
            Required resource not found

                Missing resource: {resolved_name}
            "#
        ));
    }   

    fn report_malformed_resource_declaration(&self, package_name: &str, err: &Error) {
        // value
        panic!(
            "{}",
            formatdoc!(r#"
                Resource declaration in {package_name} is malformed

                    Cause: {err}
                "#
            )
        )
    }

    fn report_malformed_resources_section(&self) {
        // TODO: ask the cargo_resources for package, value
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
            "📦"
        }
        else {
            "✨"
        };
        self.0.info(&format!("{exists} {new_sha} {output_path}"))
    }   
}

/// Resolves the static resources across the dependency tree of the crate
/// 
/// Internally it uses [cargo_resources] to do heavy lifting
pub struct Resources<'this> 
{
    /// Function used to filter the packages from which we pull the resources
    filter: Box<dyn Fn(&Package) -> bool>,
    /// Output strategy
    reporter: Reporter<'this>,
    /// If set to true enables the local resource watching
    auto_watch_local: bool,
}

impl<'this> Resources<'this>
{

    /// Create new instance of the [Resources] with defined output method
    pub fn new(
        console: &'this Console<'this>, 
        auto_watch_local: bool, 
        filter: impl Fn(&Package) -> bool + 'static,
    ) -> Self
    {
        Self{ 
            reporter: Reporter(console),
            auto_watch_local,
            filter: Box::new(filter),
        }
    }

    /// Runs the resource gathering
    /// 
    /// # Errors
    /// 
    /// Returns an error if
    /// 
    /// - `cargo-resources::collate_resources*` will return an error
    /// 
    /// # Panics
    /// 
    /// Panics if it's impossible to resolve current working directory
    pub fn run(&self) -> Result<(), String> {
        let cwd = current_dir().expect("Unable to get the current working directory.");
        let manifest_file = Utf8PathBuf::from_path_buf(cwd).unwrap().join("Cargo.toml");

        let Resources{
            reporter,
            auto_watch_local,
            filter,
        } = self;

        let collated = collate_resources_with_crate_filter(
            &manifest_file, 
            reporter,
            move |package| {
                filter(package)
            }
        )?;

        if *auto_watch_local {
            let console = reporter.0.stage("watch");
            for spec in collated {
                if spec.is_local {
                    console.info(&format!("Observing: {} - {}", spec.declaring_crate_name, spec.full_crate_path));
                    std::println!("cargo::rerun-if-changed={}", spec.full_crate_path);
                }
            }
        }

        Ok(())
    }

    /// Creates an instance of [Resources] which will vendor all the resources
    /// except the ones provided by the root crate which is being built
    /// 
    /// It's useful in the cases when you need to pull upstream resources to 
    /// generate some of the crate resources which will be later vendored by the
    /// crate
    /// 
    /// Companion function [resources_of_root_crate][Resources::resources_of_root_crate] 
    /// returns an instance of [Resources], which will vendor the resources 
    /// provided by a root crate only
    /// 
    /// # Errors
    /// 
    /// If function is called for a whole workspace.
    pub fn resources_without_root_crate<'console>(
            metadata: &Metadata, 
            console: &'console Console<'console>, 
            auto_watch_local: bool
        ) -> Result<Resources<'console>, String> {
        let Some(root) = metadata.root_package() else {
            return Err(formatdoc!(r#"
                `leptos_forge_build_script` is intended to be run for a create and not for the whole workspace.
                "#
            ))
        };
        let root_name = root.name.to_string();

        Ok(Resources::new(
            console, 
            auto_watch_local, 
            move |package: &Package| {
                package.name != root_name
            }
        ))
    }

    /// Creates an instance of [Resources] which will vendor the resources
    /// provided by the root crate only
    /// 
    /// It's useful when you need to generate some resources in based on resources
    /// provided by the upstream
    /// 
    /// Companion function [resources_without_root_crate][Resources::resources_without_root_crate]
    /// returns an instance of [Resources], which will vendor all of the resources
    /// except the one provided by the root crate
    /// 
    /// # Errors
    /// 
    /// If function is called for a whole workspace.
    pub fn resources_of_root_crate<'console>(
        metadata: &Metadata,
        console: &'console Console<'console>,
        auto_watch_local: bool,
    ) -> Result<Resources<'console>, String> {
        let Some(root) = metadata.root_package() else {
            return Err(formatdoc!(r#"
                `leptos_forge_build_script` is intended to be run for a create and not for the whole workspace.
                "#
            ))
        };
        let root_name = root.name.to_string();

        Ok(Resources::new(
            console, 
            auto_watch_local, 
            move |package: &Package| {
                package.name == root_name
            }
        ))
    }

    /// Creates an instance of [Resources] which will vendor all of the resources
    /// (upstream and from the root crate)
    ///
    /// # Arguments
    /// 
    /// - **metadata** - Metadata of the root crate, with all of the dependencies
    /// - **console** - Output formatter
    /// - **auto_watch_local** - if set to `true` [Resources] will generate the
    ///   `cargo::rerun-if-changed=` statements for all resources which are vendored
    ///   from local dependencies
    pub fn all<'console>(
        console: &'console Console<'console>,
        auto_watch_local: bool
    ) -> Resources<'console> {
        Resources::new(
            console, 
            auto_watch_local, 
            move |_: &Package| { true }
        )
    }
}