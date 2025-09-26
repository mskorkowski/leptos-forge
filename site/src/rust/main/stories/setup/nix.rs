//! Contains a section describing nix flake configuration to install rust and all required dependencies for `leptos_forge`
//! 

use forge::Section;

/// description of the [Resources] section
const NIX: &str = r############"
# Nix

The sample Nix flake configuration that can be used to develop the `leptos_forge` project looks like this

```nix
{
  inputs = {
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nixpkgs.url = "nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, fenix, flake-utils, nixpkgs }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        packages = nixpkgs.legacyPackages.${system};
        rust-toolchain = with fenix.packages.${system}; fromToolchainFile {
          file = ./rust-toolchain.toml;
          sha256 = "sha256-+9FmLhAOezBZCOziO0Qct1NOrfpjNsXxc/8I0c7BdKE=";
        };
      in {
        devShell = packages.mkShell rec {
          buildInputs = with packages; [
            clang
            cargo-shear               # cleanup unused cargo dependencies
            lld
            pkg-config 
            rust-analyzer             
            rust-toolchain
            tailwindcss_4             # tailwindcss v4 is required to build the `leptos_forge` crate
            trunk
          ];
          LD_LIBRARY_PATH = packages.lib.makeLibraryPath buildInputs;
        };
      }
    );
}
```

And `rust-toolchain.toml`

```toml
[toolchain]
channel = "stable"
components = [
    "rust-analyzer",
    "rust-src",
    "rustfmt",
]
targets = [
    "x86_64-unknown-linux-gnu",
    "wasm32-unknown-unknown"
]
```

"############;


/// Setup section
#[derive(Debug, Default, Clone, Copy)]
pub struct Nix;

impl Section for Nix {
    fn description(&self) -> &'static str {
        NIX
    }
}
