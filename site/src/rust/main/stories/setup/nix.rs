//! Contains a section describing nix flake configuration to install rust and all required dependencies for `leptos_forge`
//!

use forge::Section;

/// description of the [Resources] section
const NIX: &str = r############"
# Nix

The sample Nix flake configuration that can be used to develop the `leptos_forge` 
project looks like this

```nix
{
  description = "A basic Rust devshell for NixOS users developing Leptos_forge";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      with pkgs;
      {
        devShells.default = mkShell {
          buildInputs =
            [
              pkg-config
              trunk
              tailwindcss_4
              (rust-bin.selectLatestNightlyWith (
                toolchain:
                toolchain.default.override {
                  extensions = [
                    "rust-src"
                    "rust-analyzer"
                  ];
                  targets = [ "wasm32-unknown-unknown" ];
                }
              ))
            ]
            ++ pkgs.lib.optionals pkg.stdenv.isDarwin [
              darwin.apple_sdk.frameworks.SystemConfiguration
            ];

          shellHook = '''';
        };
      }
    );
}
```

The example configuration is based on the `flake.nix` from the `leptos` source
code.

"############;

/// Setup section
#[derive(Debug, Default, Clone, Copy)]
pub struct Nix;

impl Section for Nix {
    fn description(&self) -> &'static str {
        NIX
    }
}
