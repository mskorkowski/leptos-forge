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

In your library crate `Cargo.toml` add (you don't need to depend on the `leptos_forge_build_script` here)

```toml
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

use leptos_forge_build_script::console::{
  Console,
  ConsoleConfiguration,
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
        let _output_path = Tailwind::new(metadata, &console, None, true).
            run().
            unwrap();
    }

}
```

## Acknowledgement

Tailwind integration is based on [`cargo-resources`](https://github.com/PeteEvans/cargo-resources)
codebase.