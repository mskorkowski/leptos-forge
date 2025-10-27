# Changelog for `leptos_forge_build_script`

## 0.6.3

Rust doc updates

## 0.6.2

- Adds the `leptos_forge_build_script::Resources` to help manage the static resources.
  Internally it uses the `cargo_resources` crate ([crates.io](https://crates.io/crates/cargo-resources/1.4.1),
  [git](https://github.com/PeteEvans/cargo-resources)).

  Resources adds the ability to force rerun of the build script if any local upstream
  resources are changed.
- Makes the `package.metadata.leptos_forge.tailwind.lib` in the root crate
  
  Lack of `lib` will skip

  - generation step of tailwind `main.css`
  - default resource watching logic

  This feature is useful if, your application is simply using ui from upstream
  crates

- Updates to documentation.

## 0.6.1

Updated README.md documentation

## 0.6.0

Initial release