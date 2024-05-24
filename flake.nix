{
  description = "Rust environment";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };
  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustPkg = (pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml);
        buildDependencies = with pkgs; [
          openssl.dev
          pkg-config
          rustPkg
        ];

      in with pkgs; {
        devShells = {
          default = mkShell {
            name = "rust-env";
            buildInputs = buildDependencies ++ [
              nixfmt
            ];
          };
        };
      });
}