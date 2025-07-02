{
  description = "mkwrs - CLI chat server";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = {nixpkgs, flake-utils, rust-overlay, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ rust-overlay.overlays.default ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        rust = pkgs.rust-bin.stable.latest.default;
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "rmsgd";
          version = "0.1.0";

          src = ./.;
          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = [
            rust
            pkgs.pkg-config
          ];
        };

        devShells.default = pkgs.mkShell {
          packages = [
            rust
            pkgs.cargo
            pkgs.rust-analyzer
          ];
        };
      });
}
