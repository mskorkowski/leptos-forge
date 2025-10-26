# Leptos Forge build script library

Library to be used in your build scripts for

- Better output
  - Support for the multiline text in the cargo build scripts
  - Every lines of output starts with tags, to show where things happen
  - colors to inform you how important message is 
- Reliable tailwindcss integration
  - Build your tailwind css file from source across dependencies
  - Ability to watch for changes in your whole workspace and automatically 
    recompile css when needed
- Reliable resource provisioning
  - Uses `cargo_resources` for pulling resources from upstream crates without `include_bytes!`
  - Ability to watch for changes in your whole workspace and automatically refetch
    the resources when needed

## How to use it

### Output

In your `Cargo.toml` add 

```toml
[build-dependencies]
leptos_forge_build_script = "0.6"
```

In your build script:

```rust
use leptos_forge_build_script::console::{
  Console,
  ConsoleConfiguration,
};


fn main() {

  let console_configuration = ConsoleConfiguration::default();
  let console = Console::new("crate_name", &console_configuration);

  console.info(&"This is an info message");

  let x = 3;
  console.warn(&format!("The x = {x}"));
}
```

### Tailwind integration

Assuming your tailwind css files are located in `assets/css/lib.css` (relative to
respective `Cargo.toml`)

In your library crate `Cargo.toml` add (you don't need to depend on the `leptos_forge_build_script` here)

```toml
[package]
include=[
  "src",
  "assets"
]

[package.metadata.leptos_forge.tailwind]
# Path to the main tailwind file for your crate
#
# Lib is the tailwind `main.css` without `@import "tailwind"`. import "tailwind"
# statement will be added for you automatically.
lib="assets/css/lib.css"
```

In your application `Cargo.toml` add

In your `Cargo.toml` add 

```toml
[build-dependencies]
cargo_metadata="0.22.0"
leptos_forge_build_script = "0.6"

[package.metadata.leptos_forge.tailwind]
# Path to the main tailwind file for your crate
#
# Lib is the tailwind `main.css` without `@import "tailwind"`. import "tailwind"
# statement will be added for you automatically.
lib="assets/css/lib.css"
# Path where the compiled css file should be 
output="target/resources/css/main.css"
```

In your application build script add

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

> [!NOTE]
>
> Setting the fourth argument of `Tailwind::new` to `true` makes the `leptos_forge_build_script`
> generate [`cargo::rerun-if-changed=`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed) 
> statements, which disables the default build script behavior. Depending on 
> your other use cases it might require to adjust your build script behavior.
>
> Cargo defaults can be brought back by adding
>
> ```rust
> std::println!("cargo::rerun-if-changed=.")
> ```

#### Cargo toml configuration options for tailwind integration

```toml
[package.metadata.leptos_forge.tailwind]
lib="relative/path/to/lib.css"
output="relative/path/to/target/result.css"
watch=[]
```
- **lib** - relative path to the tailwind configuration for this crate. This 
  file will be imported from the generated tailwindcss file
- **output** - optional, relative path to the output of the tailwindcss
- **watch** - optional, list of paths which will be passed to 
  `cargo::rerun-if-changed=PATH` relative to the crate Cargo.toml. If nonempty 
  it will override the automatic path discovery.

  By default two paths are added:

  - parent folder of the `lib.rs` of the upstream crate
  - parent folder of the file pointed at in `lib` tailwind configuration option

  The defaults are conservative and probably overly broad.

### Resource provisioning

Resource provisioning in the `build_script` is a thin convenience wrapper around
the `cargo_resources` itself.

First step is to setup your crates `Cargo.toml` as described in the [`cargo-resources` documentation](https://github.com/PeteEvans/cargo-resources).

In your build script

```rust
use leptos_forge_build_script::{
  console::{
    Console,
    ConsoleConfiguration,
  },
  resources::Resources,
};

fn main() {
    // Setup printing messages to the console
    let console_configuration = ConsoleConfiguration::default();
    let console = Console::new("crate_name", &console_configuration);

    { 
        // All output from the tailwind integration will have a `[tailwind]` tag prepended
        let console = console.stage("resources");
 
        // provide all resources
        let resources = Resources::all(
          &console, 
          // If set to `true` it will generate the `cargo::rerun-if-changed=`
          // statements for all resources in local crates (same workspace)
          true
        );
        resources.run().unwrap();
    }
}
```

> [!NOTE]
>
> Setting the second argument of `Resources::all` to `true` makes the `leptos_forge_build_script`
> generate [`cargo::rerun-if-changed=`](https://doc.rust-lang.org/cargo/reference/build-scripts.html#rerun-if-changed) 
> statements, which disables the default build script behavior. Depending on 
> your other use cases it might require to adjust your build script behavior. 
>
> Cargo defaults can be brought back by adding
>
> ```rust
> std::println!("cargo::rerun-if-changed=.")
> ```

## Acknowledgement

Tailwind integration is based on [`cargo-resources`](https://github.com/PeteEvans/cargo-resources)
codebase.