//! Describes the setup process to create a new `leptos_forge` project

use forge::Section;

/// description of the [Components] section
const MAIN_DESCRIPTION: &str = r############"
# Setup
# leptos_forge

This section will guide you step by step how to create a new `leptos_forge` based project from 0 to your first tests.

## Assumptions

We assume you know how to write a simple Rust application and have a basic understanding of the Leptos framework.

## Prerequisites

Below is a list of prerequisites to run a `leptos_forge` project. Some of the dependencies are optional.

- Rust
- Cargo
- trunk
- tailwind 4 (optional)

### Setting up Rust and Cargo

You must have a working installation of Rust and cargo. You can use [`rustup`](https://rustup.rs/) script to install them.

Since we will be cross-compiling to the WebAssembly, you must install additional target `wasm32-unknown-unknown`. If
you have installed the rust using `rustup` command then you can add it using:

```sh
rustup target add wasm32-unknown-unknown
```

### Setting up trunk

In this guide we will setup `leptos_forge` project using the [`trunk`](https://trunkrs.dev/) toolchain to run the application. 
According to the [`trunk` documentation](https://trunkrs.dev/guide/getting-started/installation.html) you can install it using the following command

```sh
cargo install --locked trunk
```

### Setting up tailwind

`leptos_forge` is using a `tailwindcss` for styling components and we expose some `leptos_forge` related configuration for `tailwindcss` users.



## Setting up new project

My preferred way of setting up the project to use `leptos_forge` is to create a separate cargo project. This allows me to keep
the `leptos_forge` related code away from my main project. 

We start by creating a `Cargo.toml` file in the root of our new project

```toml
[package]
name = "my-leptos-forge-project"
edition = "2024"
publish = false # In most cases you don't need this project to be published anywhere
build="src/build.rs"

[[bin]]
name = "site"
path = "src/main.rs"

[dependencies]
leptos = { version = "0.8", features = ["csr"] }
forge = { version = "0.1.0", package = "leptos_forge", git="https://github.com/mskorkowski/leptos-forge.git" }
ui_components = { version = "0.1", package = "leptos_forge_ui_components", git="https://github.com/mskorkowski/leptos-forge.git"  }
utils_leptos = { version = "0.1", package = "leptos_forge_utils_leptos", git="https://github.com/mskorkowski/leptos-forge.git"  }
testing-library-dom = { version = "0.0.1", git="https://github.com/RustForWeb/testing-library.git", rev="05c93b5" }

console_error_panic_hook = "0.1.7"
console_log = "1.0.0"
log = "0.4.20"

[build-dependencies]
cargo_metadata="0.22.0"
cargo-resources="1.1.6"

[package.metadata.cargo_resources]
resource_root = "target/resources"
```

Let's dissect the dependencies and their roles in this project

1. **`leptos`**: This is the reactive ui framework we are using. `leptos_forge` as name implies is based on it.
2. **`forge`**: This is the core of `leptos_forge`. It provides the utilities to show your components, create documentation and help you test them.
3. **`ui_components`**: This is a collection of components provided by the `leptos_forge` so you can focus on your components instead of the control panel.
4. **`utils_leptos`**: Small library with some utilities that are being used in the `leptos_forge` project.
5. **`testing-library-dom`**: This is a workhorse library for testing your components.
6. **`console_error_panic_hook`**, **console_log** and **log**: These are used to setup error handling in your `leptos_forge` based application.

There are also two build dependencies:

1. **`cargo_metadata`**: This crate we will be used to read `Cargo.toml` of this project so we can feed this information into the `cargo-resources`.
2. **`cargo-resources`**: This crate we will be used to pull the resource files from your dependencies into your project.

> [!NOTE]
>
> Version of `testing-library-dom` which supports core features required for testing the components has not been released to crates.io yet. Until
> it is, `leptos_forge` is can't be released either and you will need to use the git version of it.

#### How your project structure will look like

In the root of your project you should have the classic `cargo` file structure with `src` and `Cargo.toml`. When you build your application
`cargo` will create a `target` directory where all the compiled code will be placed. In the next step `trunk` will create a `dist` directory
where the final `leptos_forge` based web application will be ready to be served.

```text
/
├── src/
│   ├── main.rs
│   └── ...      # rest of our `leptos_forge` source code
├── target/
|   └── ...      # compiled code and all the resources required to run it
├── dist/
|   └── ...      # here `trunk` will place the final `leptos_forge` based web application
├── Cargo.toml   # our `Cargo.toml` file
├── Trunk.toml   # our `trunk` configuration
└── index.html   # trunk requires you to have an index.html file in your project root

```

### Creating `Trunk.toml`

In the root of the project we must create a `Trunk.toml` file. This file will contain a configuration for `trunk`.

```toml

```

### Creating `index.html`

`trunk` requires us to create an `index.html` file in the root of the project. This file will be used by `trunk` as the entrypoint to the
application we are setting up.

Below is the basic `index.html` file you can use:

```html
<!doctype html>
<html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>leptos-forge - Build your own components with confidence</title>
        <link data-trunk rel="copy-dir" href="target/resources" data-target-path="resources" />
        <link data-trunk rel="css" href="target/resources/leptos_forge.css" />
        <link data-trunk rel="rust" href="Cargo.toml"/>
    </head>
    <body></body>
</html>
```

The tailwind based configuration will be shown in the tailwind section later.

### Creating the application

Finally after all of this we are ready to create our `leptos_forge` hello world application.

Now we need to create a `src/main.rs` file with the following content:

```rust
mod stories;

use leptos::prelude::*;

use forge::App;
use log::Level;
use stories::ROUTES;

/// Entrypoint of the application
pub fn main() {
    _ = console_log::init_with_level(Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| view!{
        <App routes=ROUTES logo="/resources/leptos_forge/logo/logo.svg" />
    });
}

```

The interesting parts are:

1. The file starts with `mod stories` which we will create in the second. There we will keep all of our stories to show up in the `leptos_forge` based site.
   It's not a requirement. Just a good practice.
2. We import `use forge::App` which is the main entry point of our application
3. We import `use stories::ROUTES` which we will contain the routes for the leptos. We will set it up in the moment.

### Let's write first story

First we will write an empty story for a `Counter` and add it to the menu on the left side of the site. Later we will refine it into the useful example.

```rust
use forge::RouteDef;
use forge::Story;

pub const ROUTES: &[RouteDef] = &[
    RouteDef::page::<CounterStory>("/", "Counter")
];

#[derive(Debug, Default, Clone, Copy)]
struct CounterStory {}

impl Story for CounterStory {}

```

Things which are noteworthy:

1. We've created a constant `ROUTES` which we passed in the `main.rs` into the `forge::App` component. Whenever you add the `RouteDef` to this constant
   it will show up in the menu of your application.
2. When creating an entry in the `ROUTES` we used `RouteDef::page` function. This is the most basic function to add the story to the application.
   We will cover more function later.
3. We've created a new type `CounterStory` struct. This struct derives a `Clone`, and `Copy` and `Default` traits. All three of them are required by the
   `leptos_forge`.
   1. `Default` allows you to setup initial state of the components in your story.
   2. `Copy` is required to prevent you from storying data in the story itself. You should use signals for that.

### Run it for a first time

Go into your terminal and in the root of your `leptos_forge` based project directory run

```bash
trunk serve
```

This should start a webserver listening on the `localhost:8080` where you should be able to access the just created site.

### .gitignore

If you use a `git` repository, you should add the following paths to your `.gitignore` file:

```gitignore
# Generated by cargo
# they contain the compile executables and other build artifacts
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
        MAIN_DESCRIPTION
    }
}