{
  description = "Archery Tools Lambdas";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };
        awsProfile = "AdministratorAccess-909810189216";
      in
      {
        devShells.default = with pkgs; mkShell {
          buildInputs = [
            openssl
            pkg-config
            eza
            fd
            (rust-bin.stable.latest.default.override {
              extensions = [ "rust-src" ];
              targets = [ 
                "arm-unknown-linux-gnueabihf"
                "aarch64-unknown-linux-gnu"
              ];
            })
          ];
          nativeBuildInputs = [
            cargo-lambda
          ];

          shellHook = ''
            alias ls=eza
            alias find=fd
            export AWS_PROFILE=${awsProfile}
          '';
        };
      }
    );
}
