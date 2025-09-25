//! This module describes how you can add resources to your crates and how to setup Tailwind

use forge::Section;

/// description of the [Resources] section
const RESOURCES: &str = r############"
# Resources

## Backstory

Let's assume you have an image named `logo.png` in your project that you would
like to show as the logo on top of the `leptos_forge` powered site. But this 
image will also be used in the application you are building. This means both
applications should have access to the image file. One obvious way is to copy
the image file to both locations. If it's a single image file, then why not? But
there are also icons you are using, and CSS files, and ... and so on. Copying
the files simply doesn't scale well.

## Solution

To manage resources in the project across dependencies, `leptos_forge` uses the 
[`cargo-resources`](https://github.com/PeteEvans/cargo-resources) crate.

There are two parts to the usage of the `cargo-resources` crate: providing the
resources and using them in your project.

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
     "assets/images/logo.png"
   ]
   ```

   The `package.include` list contains additional files to be packaged in the 
   Cargo crate and are not part of Rust code.

3. Add the `package.metadata.cargo_resources.provides` section to your 
   `Cargo.toml` file. This section tells `cargo-resources` how to package the 
   resources when building your application.

   ```toml
   [package.metadata.cargo_resources]
   provides = [
     { crate_path = "assets/images/logo.png", output_path="my-crate/images/logo.png", encoding="Bin" },
   ]
   ```
   
### Using the resources

To use the resources there is an additional configuration still to be made. In the application's `Cargo.toml`  you need to add
`package.metadata.cargo_resources.resource_root`. This is the path to directory relative to the `Cargo.toml` where the resources will be placed.

```toml
[package.metadata.cargo_resources]
resource_root = "target/resources"
```

This gives the final path of the resources to be concatenation of the `resource_root` and the `output_path` of the specific resource. In the case of `logo.png` file
used in examples in this section it would be `targe/resources/my-crate/images/logo.png`.

> [!TIP]
>
> Writing the `leptos_forge` we found it useful to follow the convention where `output_path` for a resource includes a name of a create to which it belongs.
> This makes it easier to manage the resources since you don't override the files by accident.

### Running the application

Before you run the application you need to run the `cargo resources` command or use the `build.rs` script. There are two ways to run the command for your `leptos_forge` based
project.

1. Let it be part of your build script (as seen in the setup)
2. Run it from using `trunk`
3. Manual running `cargo resources`

Both options have their advantages and disadvantages. Comparison is presented in the table below

|                        | Trunk                                       | Build script                                | Manual
|:-----------------------|:--------------------------------------------|:--------------------------------------------|:--------------------------------------------|
| Dependency version update | Your resources are updated automatically when you run `trunk` command | You must run `cargo build` prior to running `trunk`. We've seen the cases where `trunk` didn't respect the build script. | You must run it manually |
| nix                    | `cargo_resources` is both a lib and cli application. Unfortunate part is that it's not packaged for `nix`. | Since it's a build script, `cargo-resources` will be build as a lib and linked to the build script which makes it trivial to use. | The same issue as trunk has. |
| Other build configurations | This works only for trunk. Any other build setup will fail | This works for any setup which runs a `cargo build` internally. | You must not forget to run it |

If you followed the [create project guide](/guides/create_project) then you have a build script configured already.

#### Using `trunk` to bundle the resources

In your `Trunk.toml` add the following hook:

```toml
[[hooks]]
stage = "pre_build"
command = "cargo"
command_arguments = ["resources"]
```

#### Using build script

If you followed the [create project](/guides/create_project) this is a configuration which was suggested to you.

To make the build script approach work you need to make following changes in the `Cargo.toml`

```toml
[build-dependencies]
cargo_metadata="0.22.0"
cargo-resources="1.1.6"
```

This will add the necessary dependencies for the build script approach.

Now beside the root of your crate you need to add the `build.rs` file

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

`leptos_forge` uses Tailwind 4 to manage the css styles.

To use the Tailwind within the `leptos_forge` based application you need to adjust the Tailwind `main.css` file to include the necessary files and
do small changes to the `index.html` file used by the `trunk`.

## Setting up `cargo-resources`

I assume you have `cargo-resources` set and running. If not you can follow either [setup](/guides) or [resources](/guides/resources) chapter. For the
rest of this chapter I will assume that in your `Cargo.toml` you have the following configuration

```toml
[package.metadata.cargo_resources]
resource_root = "target/resources"
```

## Setting up Tailwind

I assume you have installed Tailwind and have the `assets/css/main.css` file in your project as entry point for your Tailwind. If your `main.css`
is somewhere else you will need to adjust the paths accordingly.

`leptos_forge` provides two files in for you to include in the `main.css` file. 

- **`leptos_forge.css`** - css file which has all the necessary styles to run the `leptos_forge`
- **`common.css`** - tailwind css configuration and some animation related utilities.

To integrate `leptos_forge` with your project's Tailwind setup you need to import both of the files in your `main.css` file by adding

```css
@import "../../target/resources/leptos_forge/common.css";
@import "../../target/resources/leptos_forge/leptos_forge.css";
```

## Setting up trunk

Trunk has builtin integration with Tailwind. Your `index.html` file needs to be slightly adjusted compared to the [create project](/guides/create_project)

```html
<!doctype html>
<html lang="en">
    <head>
        <meta charset="UTF-8">
        <title>My leptos_forge site</title>
        <link data-trunk rel="copy-dir" href="target/resources" data-target-path="resources" />
        <!-- <link data-trunk rel="css" href="target/resources/leptos_forge.css" /> -->      <!-- remove this line -->
        <link data-trunk rel="tailwind-css" href="assets/css/main.css" />                    <!-- add this line -->
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