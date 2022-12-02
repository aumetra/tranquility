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
                postgresql_14
                nixpkgs-fmt
                (rust-bin.fromRustupToolchainFile ./rust-toolchain.toml)
                sqlx-cli
              ] ++ lib.optional stdenv.isDarwin [ libiconv darwin.apple_sdk.frameworks.SystemConfiguration ];
          };
        });
}
