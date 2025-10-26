# `leptos_forge` Changes

## 0.6.2

### `leptos_forge_build_script`

- Adds the reliable resource management wrapping `cargo_resources` crate
- Makes `lib` in `Cargo.toml` configuration optional
- Better documentation
- Updated website related to the tailwind and resources bundling

## 0.6.1

### `leptos_forge_build_script`

Better README.md documentation

## 0.6.0

First crates.io release

### `leptos_forge_build_script`

- Reliable tailwindcss integration via build script

### `leptos_forge_utils`

Generic leptos free utils

### `leptos_forge_utils_leptos`

Generic utilities to be used together with `leptos`

- **URwSignal** - generic purpose read/write signal implementation
- css class swapping
  - **use_swap_class** - to replace one set of css classes with another 
  - **use_add_class** - to add a set of css classes
  - **use_remove_class** - to remove a set of css classes
- **StoredRef** - wrapper type which allows to keep the `Element` in the leptos store

## 0.5.* and earlier versions

Marek's personal use versions.
