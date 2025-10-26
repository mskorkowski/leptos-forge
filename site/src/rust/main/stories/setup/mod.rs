//! Describes the setup process to create a new `leptos_forge` project
//!
//! All setup guides should follow standard Cargo project structure and not the `leptos_forge`
//! one.
pub mod adding_tests;
pub mod nix;
pub mod refine_story;
pub mod resources;

use forge::RouteDef;
use forge::Section;

/// description of the [Components] section
const SETUP: &str = r############"
# Setup
# leptos_forge

This section will guide you step by step on how to create a new working `leptos_forge`-based
project. The following section will guide you through the rest of the workflow required to 
create and test your components.

## Assumptions

We assume you know how to write a simple Rust application and have a basic understanding
of the Leptos framework.

## Prerequisites

Below is a list of prerequisites to run a project using `leptos_forge`.

- Rust
- Cargo
- Trunk
- Tailwind 4

Sample `nix` configuration can be found in the [`nix` section](/guides/nix).

### Setting up Rust and Cargo

You must have a working installation of Rust and Cargo. You can use the [`rustup`](https://rustup.rs/)
 script to install them.

Since we will be cross-compiling to the WebAssembly, you must install the additional target 
`wasm32-unknown-unknown`. If you installed the Rust using `rustup` you can add it running

```sh
rustup target add wasm32-unknown-unknown
```

### Setting up trunk

In this guide we will set up a project using the [`trunk`](https://trunkrs.dev/) to run the application.
According to the [`trunk` documentation](https://trunkrs.dev/guide/getting-started/installation.html), 
you can install it using the following command

```sh
cargo install --locked trunk
```

### Setting up Tailwind

`leptos_forge` uses `tailwindcss` for styling components. We provide some `leptos_forge` related 
configuration for Tailwind users. The Tailwind installation instructions can be found on 
[the official Tailwind website](https://tailwindcss.com/docs/installation/tailwind-cli).

## Setting up a new project

My preferred way of setting up the project to use `leptos_forge` is to create a separate 
Cargo crate in the workspace. This allows me to keep the `leptos_forge` related code away 
from my main project. 

We start by creating a `Cargo.toml` file in the root of our new project:

```toml
[package]
name = "my-leptos-forge-project"
edition = "2024"
publish = false # In most cases you don't need this project to be published anywhere

[[bin]]
name = "site"
path = "src/main.rs"

[dependencies]
leptos = { version = "0.8", features = ["csr"] }
forge = { version = "0.6", package = "leptos_forge", git="https://github.com/mskorkowski/leptos-forge.git" }
ui_components = { version = "0.5", package = "leptos_forge_ui_components", git="https://github.com/mskorkowski/leptos-forge.git"  }
utils_leptos = { version = "0.6", package = "leptos_forge_utils_leptos" }
testing-library-dom = { version = "0.0.1", git="https://github.com/RustForWeb/testing-library.git", rev="05c93b5" }

console_error_panic_hook = "0.1.7"
console_log = "1.0.0"
log = "0.4.20"

[build-dependencies]
cargo_metadata="0.22.0"
cargo-resources="1.1.6"
leptos_forge_build_script="0.6"

[package.metadata.cargo_resources]
resource_root = "target/resources"

[package.metadata.leptos_forge.tailwind]
lib="assets/css/lib.css"
output="target/resources/css/main.css"
```

Let's dissect the dependencies and their roles in this project:

1. **`leptos`**: This is the reactive UI framework we are using. `leptos_forge` as its name 
   implies is based on it.
2. **`forge`**: This is the core of `leptos_forge`. It provides utilities to show your 
   components, create documentation, and help you test them.
3. **`ui_components`**: This is a collection of components provided by `leptos_forge` so you
   can focus on your components instead of the control panel.
4. **`utils_leptos`**: A small library with some utilities used in the `leptos_forge` project.
5. **`testing-library-dom`**: This is the workhorse library for testing your components.
6. **`console_error_panic_hook`**, **`console_log`**, and **`log`**: These are used to 
   set up error handling in your `leptos_forge`-based application.

There are also three build dependencies:

1. **`cargo_metadata`**: This crate is used to read the `Cargo.toml` of this project so we can
  feed this information into `cargo-resources`.
2. **`cargo-resources`**: This crate is used to pull resource files from your dependencies into
   your project.
3. **`leptos_forge_build_script`** - is the library intended to be used as part of
   Cargo build script. Provides a seamless tailwind integration.

> [!NOTE]
>
> The version of `testing-library-dom` which supports core features required for testing 
> components has not been released to crates.io yet.

#### How project structure will look like

In the root of your project you should have the classic `cargo` file structure with `src` and 
`Cargo.toml`. When you build your application `cargo` will create a `target` directory where
all the compiled code will be placed. In the next step `trunk` will create a `dist` directory
where the final `leptos_forge` based web application will be ready to be served.

```text
/
├── src/
│   ├── main.rs
│   └── ...      # rest of source code
├── target/
│   └── ...      # compiled code and all the resources required to run it
├── dist/
│   └── ...      # here `trunk` will place the final `leptos_forge` based web application
├── build.rs     # build script to copy resources from upstream projects
├── Cargo.toml   # our `Cargo.toml` file
├── Trunk.toml   # our `trunk` configuration
└── index.html   # trunk requires you to have an index.html file in your project root
```

### Build script

`leptos_forge` provides some `js`, `css` and images that can be used in your `leptos_forge` 
based application. To automate the process, we use the `build.rs` script. This script will
search for resources in the upstream projects and copy them into the `target/resources` directory.
In depth explanation about the resource management can be found in the 
[Resources](/guides/resources) section.

`target/resources` directory has been set up in the last line of the `Cargo.toml` file. For more
details about configuration options for the `cargo-resources` crate, please refer to the 
[Cargo Resources documentation](https://github.com/PeteEvans/cargo-resources).

The `build.rs` script should look like this:

```rust
use leptos_forge_build_script::console::Console;
use leptos_forge_build_script::console::ConsoleConfiguration;
use leptos_forge_build_script::tailwind::Tailwind;
use cargo_metadata::CargoOpt;
use cargo_metadata::Metadata;
use cargo_metadata::MetadataCommand;
use cargo_metadata::camino::Utf8PathBuf;
use cargo_resources::collate_resources_with_reporting;
use cargo_resources::reporting::ReportingTrait;
use std::env::current_dir;

fn main() {
    let console_configuration = ConsoleConfiguration::default();
    let console = Console::new("site", &console_configuration);

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

    // Collate resources from the crate's dependencies.
    collate_resources(&manifest_file).expect("There was an error during bundling of the resources");
}
```

### Creating `index.html` for the `trunk`

`trunk` requires us to create an `index.html` file in the root of the project. This file will
be used by `trunk` as the entrypoint to the application we are setting up.

Below is the basic `index.html` file you can use:

```html
<!doctype html>
<html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>My leptos_forge site</title>
        <meta name="viewport" content="width=device-width, height=device-height, initial-scale=1, interactive-widget=overlays-content" />
        <link data-trunk rel="rust" href="Cargo.toml"/>
        <link data-trunk rel="copy-dir" href="target/resources" data-target-path="resources" />
        <link rel="stylesheet" href="/resources/css/main.css" />
    </head>
    <body></body>
</html>
```

Using Tailwind in your `leptos_forge` application is described in [Tailwind chapter](/guides/tailwind).

### Creating the first story

Finally, after all of this, we are ready to create our `leptos_forge` hello world application.
We will create the smallest example which we will refine later in the guide.

Now we need to create a `src/main.rs` file with the following content:

```rust
use log::Level;
use leptos::prelude::*;
use forge::{App, RouteDef, Story};

#[derive(Debug, Default, Clone, Copy)]
struct CounterStory {}

impl Story for CounterStory {}

/// Entrypoint of the application
pub fn main() {
    _ = console_log::init_with_level(Level::Debug);
    console_error_panic_hook::set_once();

   let routes = vec![
      RouteDef::story::<CounterStory>("/", "Counter")
   ];

    mount_to_body(move || view!{
        <App routes logo="/resources/leptos_forge/logo/logo.svg" />
    });
}
```

Things which are noteworthy:

1. We use `forge::App`, which is the main entry point of our `leptos_forge` application.
2. We've created a value `routes` which we passed in the `forge::App` component.
   This value is the top level routing structure of the application.
2. When creating an entry in the `routes` we used `RouteDef::story` function. 
   This function allows to add stories about components into the `leptos_forge` 
   based site.
3. We've created a new type `CounterStory` struct. This struct derives `Clone`, 
   `Copy`, and `Default` traits. All three are required by `leptos_forge`.
   `leptos_forge` uses the `Default` trait to setup initial state of the components
   in your story.

### Run it for the first time

Go into your terminal, and in the root of your `leptos_forge` based project directory. Now we
need to run the build process to allow the `cargo-resources` to handle the assets. In depth
explanation can be found in the [Resources](/guides/resources) section.

```bash
cargo build
```

> [!NOTE]
>
> Any time you update your dependencies in the `Cargo.toml` which provide resources, you will
> need to rebuild the project using `cargo build`.

Now we can finally start the server with our new `leptos_forge` based site.

```bash
trunk serve
```

This should start a web server listening on the `localhost:8080` where you should be able to
access the just-created site.

If everything went well you should see the view like this:

![Initial view](/resources/leptos_forge_site/images/guides/first_run.png)

On the left side under the `leptos_forge` logo there is a `menu`. Currently, only the `Counter`
is there.

On the right side, which is currently empty, is a control panel. Here you will find the controls to change your components.

In the center, the gray area is the place where your components will show up. In the next step
we will add the `Counter` component there.

Below the gray area there is a documentation panel. At this moment you can find the **New Story**
help. Any time you create a new element to show up in the app we will show you there what are
your next steps to make it work.

### .gitignore

If you use a `git` repository, you should add the following paths to your `.gitignore` file:

```gitignore
# Generated by cargo
# they contain the compiled executables and other build artifacts
target

# Trunk dist files
**/dist
```

"############;

/// Setup section
#[derive(Debug, Default, Clone, Copy)]
pub struct Setup;

impl Section for Setup {
    fn description(&self) -> &'static str {
        SETUP
    }

    fn subroutes(&self) -> Vec<RouteDef> {
        vec![]
    }
}
