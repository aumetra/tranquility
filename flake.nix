{
  description = "Tranquility";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-22.11";
    flake-utils = {
      url = "github:numtide/flake-utils";
    };
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { self, nixpkgs, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
          };
        in
        {
          devShell = pkgs.mkShell {
            buildInputs = with pkgs;
              [
                nixpkgs-fmt
                (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
              ];
          };
        });
}
