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
          sha256 = "sha256-IyelwCDfJ/ASxSMH6tKFO17DJybfI9Hl05i0jIXK898=";
        };
      in {
        devShell = packages.mkShell rec {
          buildInputs = with packages; [
            trunk
            tailwindcss_4
            lld
            rust-toolchain
            pkg-config
            rust-analyzer
            mold-wrapped
            clang
          ];
          LD_LIBRARY_PATH = packages.lib.makeLibraryPath buildInputs;
        };
      }
    );
}
# {
#   description = "A basic Rust devshell for NixOS users developing Leptos";

#   inputs = {
#     nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
#     rust-overlay.url = "github:oxalica/rust-overlay";
#     flake-utils.url = "github:numtide/flake-utils";
#   };

#   outputs =
#     {
#       nixpkgs,
#       rust-overlay,
#       flake-utils,
#       ...
#     }:
#     flake-utils.lib.eachDefaultSystem (
#       system:
#       let
#         overlays = [ (import rust-overlay) ];
#         pkgs = import nixpkgs {
#           inherit system overlays;
#         };
#       in
#       with pkgs;
#       {
#         devShells.default = mkShell {
#           buildInputs =
#             [
#               gcc
#               glib
#               openssl
#               pkg-config
#               cacert
#               cargo-make
#               trunk
#               (rust-bin.selectLatestNightlyWith (
#                 toolchain:
#                 toolchain.default.override {
#                   extensions = [
#                     "rust-src"
#                     "rust-analyzer"
#                   ];
#                   targets = [ "wasm32-unknown-unknown" ];
#                 }
#               ))
#               tailwindcss_4
#               rust-analyzer
#               clang
#             ]
#             ++ pkgs.lib.optionals pkg.stdenv.isDarwin [
#               darwin.apple_sdk.frameworks.SystemConfiguration
#             ];

#           shellHook = '''';
#         };
#       }
#     );
# }