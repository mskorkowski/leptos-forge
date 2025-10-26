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
   version = "0.6.0"
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

#### Manual bundling of the resources

Every time you wish to run the application, after you build it you need to run the 

```bash
cargo resources
```

So the running involves three steps

1. resources - bundle the resources
2. build - build the application
3. run - run the app

#### Using `trunk` to bundle the resources

> [!NOTE]
> # Trunk and cargo-resources limitations 
>
> Due to limitations in Trunk and cargo-resources (mostly the second one) there 
> are two solutions
> 
> 1. Use trivial hack to always rerun the `build_script`
> 2. Accept that if you update the resources the cargo build script caching algorithm
>    will prevent the build script to rerun
>
> There is an open ticket which will allow us to remove the issue:
> [

In your `Trunk.toml` add the following hook:

```toml
[[hooks]]
stage = "build"
command = "cargo"
command_arguments = ["resources"]
```

#### Using build script

If you followed the [create project](/guides/create_project), this configuration
was suggested to you.

To make the build script approach work, you need to make the following changes
in the `Cargo.toml`:

```toml
[build-dependencies]
cargo_metadata="0.22.0"
cargo-resources="1.1.6"
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
# Tailwind

`leptos_forge` uses Tailwind 4 to manage the CSS styles.

To use the Tailwind within the `leptos_forge` based application you need to
adjust the Tailwind `main.css` file to include the necessary files and make
small changes to the `index.html` file used by the `trunk`.

## Adding dependencies for the build script

In your `Cargo.toml` in the build-dependencies section add

```toml
[build-dependencies]
...
build-print.workspace = true
```

> [!NOTE]
> ##### `build-print`
>
> `build-print` crate works around the `cargo` limitations on message formatting
> and allows you to show more then `warning` and `error` (and hides the ugly syntax)

## Setting up `cargo-resources`

I assume you have `cargo-resources` set up and running. If not you can follow
either [setup](/guides) or [resources](/guides/resources) chapter. For the rest
of this chapter I will assume that in your `Cargo.toml` you have the following
configuration

```toml
[package.metadata.cargo_resources]
resource_root = "target/resources"
```

## Building css

In your build script after collating the resource add

```
/// Description in case of tailwind execution failure
const TAILWIND_FAILURE: &str = "Build script can't find the tailwindcss cli. Please check your tailwind installation";

/// Simple helper function to print the multiline string to the cargo output
fn error<S: ToString>(s: S) {
    for line in s.to_string().split("\n") {
            error!("{line}");
    }
}

/// Simple helper function to print the multiline string to the cargo output
fn info<S: ToString>(s: S) {
    for line in s.to_string().split("\n") {
            info!("{line}");
    }
    
}

/// Simple helper function to print a normal multiline test to the cargo output
fn println<S: ToString>(s: S) {
    for line in s.to_string().split("\n") {
            println!("{line}");
    }
}

fn main() {
    ...

    // Collate resources from the crate's dependencies.
    collate_resources(&manifest_file).expect("There was an error during bundling of the resources");

    // After all dependencies are in place (inside of the `package.metadata.cargo_resources.resource_root`
    // path we can run tailwind to build them

    let output = Command::new("tailwindcss")
        .args(vec![
            "--input", "src/css/main/main.css",
            "--output", "target/resources/leptos_forge_site/main.css",
            "--optimize" // remove duplicated classes, ...
        ])
        .output()
        .expect(TAILWIND_FAILURE);

    if !output.status.success() {
        println(format!("Tailwind Stopped with an {}", output.status));
        error("");
        error("--------[ STDOUT ]------------------------");
        error("");
        println(String::from_utf8_lossy(&output.stdout));
        error("");
        error("--------[ STDERR ]------------------------");
        error("");
        error(String::from_utf8_lossy(&output.stderr));

    }
    else {
        info("[SITE] Tailwind run successfully");
    }

}


```

## Setting up Tailwind

I assume you have installed Tailwind and have the `assets/css/main.css` file in
your project as the entry point for your Tailwind. If your `main.css` is
somewhere else you will need to adjust the paths accordingly.

`leptos_forge` provides two files in for you to include in the `main.css` file. 

- **`leptos_forge.css`** - css file which has all the necessary styles to run the `leptos_forge`
- **`common.css`** - tailwind css configuration and some animation related utilities.

To integrate `leptos_forge` with your project's Tailwind setup you need to import both of the files in your `main.css` file by adding

```css
@import "../../target/resources/leptos_forge/common.css";
@import "../../target/resources/leptos_forge/leptos_forge.css";
```

## Setting up trunk

Trunk has built-in integration with Tailwind. Your `index.html` file needs to
be slightly adjusted compared to the [create project](/guides/create_project)

```html
<!doctype html>
<html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>My leptos_forge site</title>
        <link data-trunk rel="copy-dir" href="target/resources" data-target-path="resources" />
        <link rel="stylesheet" href="/resources/leptos_forge_site/main.css" />
        <link data-trunk rel="rust" href="Cargo.toml"/>
    </head>
    <body></body>
</html>
```



"############;

/// How to setup tailwind
#[derive(Debug, Default, Clone, Copy)]
pub struct Tailwind;

impl Section for Tailwind {
    fn description(&self) -> &'static str {
        TAILWIND
    }
}
