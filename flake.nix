{
  description = "Rust development environment (https://github.com/oxalica/rust-overlay/blob/ab69b67fac9a96709fbef0b899db308ca714a120/README.md#use-in-devshell-for-nix-develop)";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
      in
      {
        devShells.default = with pkgs; mkShell {
          packages = [
            (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
            openssl
            pkg-config
            cargo-deny
            # cargo-llvm-cov # Marked as broken in Nix, https://github.com/NixOS/nixpkgs/blob/e2dd4e18cc1c7314e24154331bae07df76eb582f/pkgs/development/tools/rust/cargo-llvm-cov/default.nix#L97
            cargo-edit
            cargo-insta
            cargo-fuzz
            cargo-watch
            cargo-msrv
            rust-analyzer
            pandoc
            gnumake
          ];

          shellHook = ''
            # Marked as broken in Nix; I don't have the time figuring out Rust nightly,
            # LLVM tooling, and Nix (overlays) in combination 🤕
            cargo install cargo-llvm-cov
          '';
        };
      }
    );
}
