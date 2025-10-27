//! This module describes how you can add resources to your crates and how to setup Tailwind

use forge::Section;

/// description of the [Resources] section
const RESOURCES: &str = r############"
# Resources

Let's assume you have an image named `logo.png` in your project that you would
like to show as the logo on top of the `leptos_forge` powered site. But this 
image will also be used in the application you are building. This means both
applications should have access to the image file. One obvious way is to copy
the image file to both locations. If it's a single image file, then why not? But
there are also icons you are using, and CSS files, and ... and so on. Copying
the files simply doesn't scale well.

To manage resources in the project across dependencies, `leptos_forge` uses the 
[`cargo-resources`](https://github.com/PeteEvans/cargo-resources) crate.

We've chosen the `cargo-resource` because 

- it doesn't bloat wasm file
- it nicely integrates with `cargo`
- you can call it from build script
- We found Manganis harder to integrate and it currently requires `dx`

## Using `cargo-resources`

There are two parts to the usage of the `cargo-resources` crate: 

1. providing the resources 
2. using them in your project.

### Providing the Resources

Providing the resource has three steps.

1. Place a file somewhere in your project. For example `assets/images/logo.png`.
2. Add the file to the `package.include` list in your `Cargo.toml` file.

   ```toml
   [package]
   name = "my-crate"
   version = "0.1.0"
   edition = "2024"
   include = [
     "assets",
     "src"
   ]
   ```

   The `package.include` list contains additional files to be packaged in the 
   Cargo crate and are not part of Rust code. This is a standard Cargo part.

3. Add the `package.metadata.cargo_resources.provides` section to your 
   `Cargo.toml` file. This section tells `cargo-resources` how to package the 
   resources when building your application.

   ```toml
   [package.metadata.cargo_resources]
   provides = [
     { crate_path = "assets/images/logo.png", output_path="my-crate/images/logo.png", encoding="Bin" },
   ]
   ```

   - `crate_path` - is the path in the crate source code where the file is. This
     path must be present in the Cargo's `package.include`. Otherwise your 
     dependencies won't have an ability to read your resource files since they
     won't be a part of your crate.
   - `output_path` is a path relative to the assets root directory. The location
     off assets root directory is defined by the downstream crates
   
### Using the resources

To use the resources there is an additional configuration still to be made. In
the application's `Cargo.toml` you need to add `package.metadata.cargo_resources.resource_root`. 
This is the path to directory relative to the `Cargo.toml` where the assets 
will be placed.

```toml
[package.metadata.cargo_resources]
resource_root = "target/resources"
```

To know where your resource would end up in the directory structure you need to
concatenate the `resource_root` with the asset `output_path`.

In the case of `logo.png` file used in examples in this section it would be 
`target/resources/my-crate/images/logo.png`.

> [!TIP]
>
> Writing the `leptos_forge` we found it useful to follow the convention where 
> `output_path` for a resource includes a name of a crate to which it belongs.
> This makes it easier to manage the resources since you don't override the files
> by accident.

### Running the application

#### Using `leptos_forge_build_script`

`leptos_forge_build_script` will give you the best experience, especially if you
use a cargo workspace in your project.

If you followed the [create project](/guides/create_project), this configuration
was suggested to you. If not, you can follow the steps below

To make the build script approach work, you need to make the following changes
in the `Cargo.toml`:

```toml
[build-dependencies]
leptos_forge_build_script.workspace = "0.6"
cargo_metadata = "0.23"
```

This will add the necessary dependencies for the build script approach.

Now beside the root of your crate you need to add the `build.rs` file:

```rust
use build_script::console::Console;
use build_script::console::ConsoleConfiguration;
use build_script::resources::Resources;

fn main() {
    let console_configuration = ConsoleConfiguration::default();
    let console = Console::new("site", &console_configuration);
    
    // Uncomment the line below, if you depend on the default cargo behavior of
    // rerunning the build script every time anything changes in your project
    //std::println!("cargo::rerun-if-changed=.");

    {
        let console = console.stage("resources");
        let resources = Resources::all(&console, true);
        resources.run().unwrap();
    }
}
```

##### Integrating with Trunk

> [!NOTE]
> ##### Trunk limitations
>
> Trunk doesn't handle the well the resources which are generated during the 
> build phase. Due to this limitation to have reliable integration with Trunk
> you need to run an extra `cargo check` during the `pre_build` phase.
>
> If you followed `Create project` guide you already have it.

In your `Trunk.toml` add the following hook:

```toml
[[hooks]]
stage = "pre_build"
command = "cargo"
command_arguments = ["check"]
```

##### Using upstream resources to generate the resources for your crate

Let's assume the crate in your upstream provides a sass stylesheets, which you
need to compile together with your project sass files so you can generate the 
final css.

The steps needed

1. Provide upstream resources
2. Run sass compilation to get the final css
3. Provide local resources to move the final css to correct place

Solution

```rust
use std::env::current_dir;

use cargo_metadata::CargoOpt;
use cargo_metadata::Metadata;
use cargo_metadata::MetadataCommand;
use cargo_metadata::camino::Utf8PathBuf;

use build_script::console::Console;
use build_script::console::ConsoleConfiguration;
use build_script::resources::Resources;


fn main() {
    let console_configuration = ConsoleConfiguration::default();
    let console = Console::new("site", &console_configuration);
    
    // Uncomment the line below, if you depend on the default cargo behavior of
    // rerunning the build script every time anything changes in your project
    //std::println!("cargo::rerun-if-changed=.");

    let cwd = current_dir().unwrap();
    let manifest_file = Utf8PathBuf::from_path_buf(cwd).unwrap().join("Cargo.toml");
    let mut metadata_cmd = MetadataCommand::new();
        let metadata: Metadata = metadata_cmd
            .manifest_path(&manifest_file)
            .features(CargoOpt::AllFeatures)
            .exec()
            .unwrap();

    {
        // 1. Provide upstream resources
        let console = console.stage("resources");
        Resources::resources_without_root_crate(&metadata, &console, true).
            run().
            unwrap();

        // 2. Run sass
        let mut sass: Command = Command::new("sass");
        sass
            .args(vec![
                "assets/saas/input.scss",
                "target/resources/css/output.css",
            ]);

        // 3. Provide local resources to move the final css to correct place
        Resources::resources_of_root_crate(&metadata, &console, true).
            run().
            unwrap();
    }
}
```

#### Manual bundling of the resources

Every time you wish to run the application, after you build it you need to run the 

```bash
cargo resources
```

So the running involves three steps

1. resources - bundle the resources
2. build - build the application
3. run - run the app

#### Using `cargo_resources` from build script

To make the build script approach work, you need to make the following changes
in the `Cargo.toml`:

```toml
[build-dependencies]
cargo_metadata="0.23.0"
cargo-resources="1.4.1"
```

This will add the necessary dependencies for the build script approach.

Now beside the root of your crate you need to add the `build.rs` file:

```rust
use std::env::current_dir;
use cargo_metadata::camino::Utf8PathBuf;
use cargo_resources::collate_resources;

fn main() {
    let cwd = current_dir().unwrap();
    let manifest_file = Utf8PathBuf::from_path_buf(cwd).unwrap().join("Cargo.toml");

    // Collate resources from the crate's dependencies.
    collate_resources(&manifest_file).expect("There was an error during bundling of the resources");
}
```

If you decide to use this approach, you can use code in `leptos_forge_build_script::resources`
module as an example how to integrate with `cargo_resources`.


"############;

/// Setup section
#[derive(Debug, Default, Clone, Copy)]
pub struct Resources;

impl Section for Resources {
    fn description(&self) -> &'static str {
        RESOURCES
    }
}

/// description of the [Tailwind] section
const TAILWIND: &str = r############"
# Integrating with tailwind

To integrate with Tailwind, you can use `leptos_forge_build_script` crate. In your
application crate add

```toml
[build-dependencies]
leptos_forge_build_script = 0.6

[package.metadata.leptos_forge.tailwind]
# Cargo.toml relative path to your tailwind file without `@import "tailwindcss"` statement
lib="assets/css/lib.css"
# Cargo.toml relative path where you would like to place the file build by tailwindcss 
output="target/resources/css/main.css"
```

In your build script add

```rust
use std::env::current_dir;

use cargo_metadata::{
  CargoOpt,
  Metadata,
  MetadataCommand,
  camino::Utf8PathBuf,
}

use leptos_forge_build_script::{
  console::{
    Console,
    ConsoleConfiguration,
  },
  tailwind::Tailwind,
};

fn main() {
    // Setup printing messages to the console
    let console_configuration = ConsoleConfiguration::default();
    let console = Console::new("crate_name", &console_configuration);

    // Read the cargo manifest
    let cwd = current_dir().unwrap();
    let manifest_file = Utf8PathBuf::from_path_buf(cwd).unwrap().join("Cargo.toml");
    let mut metadata_cmd = MetadataCommand::new();
    let metadata: Metadata = metadata_cmd
        .manifest_path(&manifest_file)
        .features(CargoOpt::AllFeatures)
        .exec()
        .unwrap();

      
    { 
        // All output from the tailwind integration will have a `[tailwind]` tag prepended
        let console = console.stage("tailwind");
      
        // Run tailwind integration
        // 
        // 
        let _output_path = Tailwind::new(
            metadata, 
            &console, 
            // Override to any path you wish the main tailwind file should be generated
            // by default it's placed under the path created using algorithm below
            //
            // 1. `OUT_DIR` env variable
            // 2. To the path from `1` append `leptos_forge/build_script/tailwind`
            //    to make it separate from the rest of the output
            //
            None, 
            // If set to `true` it generates `cargo::rerun-if-changed=` statements
            // for the local files (same workspace) scanned by tailwind.
            //
            // watching behavior can be fine tuned in your Cargo.toml
            // `package.metadata.leptos_forge.tailwind.watch` configuration
            true
        ).
            run().
            unwrap();
    }
}
```

## Using in ui libraries

If you have a crate which will provide user interface and you use the tailwindcss
to style it in the `Cargo.toml` of your UI lib add

```toml
[package]
# Add include directive to your crate, together with your tailwindcss assets
include=[
    "src",
    "assets",
]

[package.metadata.leptos_forge.tailwind]
# Cargo.toml relative path to your tailwind file without `@import "tailwindcss"`
# statement. Tailwind will use it to scan your source files.
lib="assets/css/lib.css"
```

Thats it.

> [!NOTE]
>
> You don't need to add the dependency to the `leptos_forge_build_script` to the 
> UI lib crate.

## Using with Trunk

Assuming that in your application Cargo.toml you have `package.metadata.leptos_forge.tailwind.output="target/resources/css/main.css"`

Your `index.html` can look like this

```html
<!doctype html>
<html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>My leptos_forge site</title>
        <link data-trunk rel="copy-dir" href="target/resources" data-target-path="resources" />
        <link rel="stylesheet" href="/resources/css/main.css" />
        <link data-trunk rel="rust" href="Cargo.toml"/>
    </head>
    <body></body>
</html>
```

Because of the limitation of the Trunk, you need to create a `Trunk.toml` file
with content

```toml
# Ensures that resources are in place before bundling will take place
# This is related to the limitation of the trunk
[[hooks]]
stage = "pre_build"    # When to run hook, must be one of "pre_build", "build", "post_build"
command = "cargo"      # Command to run
command_arguments = [  # Arguments to pass to command
    "check",
]
```

> [!NOTE]
> ##### Why `cargo check` is needed?
>
> Trunk runs the building of the application and bundling of the resources in
> parallel. That means it's totally possible for the bundling to finish the process
> before the build script finishes the execution and often it does.
> 
> As a workaround sample configurations contains the `cargo check` in pre_build
> state. This ensures that all files are present before the build even starts.

"############;

/// How to setup tailwind
#[derive(Debug, Default, Clone, Copy)]
pub struct Tailwind;

impl Section for Tailwind {
    fn description(&self) -> &'static str {
        TAILWIND
    }
}
