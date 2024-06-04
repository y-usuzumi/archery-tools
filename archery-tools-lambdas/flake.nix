{
  description = "Nix Flake for Archery tools lambdas";

  inputs = {
    nixpkgs.url = "flake:nixpkgs";
    flake-parts.url = "flake:flake-parts";
    systems.url = "flake:systems";
    rust-overlay.url = "github:oxalica/rust-overlay";

    # Dev tools
    treefmt-nix.url = "github:numtide/treefmt-nix";
  };

  outputs = inputs:
    inputs.flake-parts.lib.mkFlake { inherit inputs; } {
      systems = import inputs.systems;
      imports = [
        inputs.treefmt-nix.flakeModule
      ];
      perSystem = { config, self', pkgs, lib, system, ... }:
        let
          cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
          nonRustDeps = with pkgs; [
            libiconv
            awscli2
          ];
          awsProfile = "AdministratorAccess-909810189216";
        in
        {
          # Rust package
          packages.default = pkgs.rustPlatform.buildRustPackage {
            inherit (cargoToml.package) name version;
            src = ./.;
            cargoLock.lockFile = ./Cargo.lock;
          };

          # Rust dev environment
          devShells.default = pkgs.mkShell {
            inputsFrom = [
              config.treefmt.build.devShell
            ];
	    # Not using just for now
            # shellHook = ''
            #   # For rust-analyzer 'hover' tooltips to work.
            #   export RUST_SRC_PATH=${pkgs.rustPlatform.rustLibSrc}

            #   echo
            #   echo "🍎🍎 Run 'just <recipe>' to get started"
            #   just
            # '';
            shellHook = ''
            export AWS_PROFILE=${awsProfile}
            '';
            buildInputs = nonRustDeps;
            nativeBuildInputs = with pkgs; [
              just
              rustc
              cargo
              cargo-lambda
              cargo-watch
              rust-analyzer
            ];
          };

          # Add your auto-formatters here.
          # cf. https://numtide.github.io/treefmt/
          treefmt.config = {
            projectRootFile = "flake.nix";
            programs = {
              nixpkgs-fmt.enable = true;
              rustfmt.enable = true;
            };
          };
        };
    };
}
