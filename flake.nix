{
  description = "Archery DB flake";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
      pkgs = import nixpkgs { inherit system; };
      awsProfile = "AdministratorAccess-909810189216";
      in rec {
        devShell = pkgs.mkShell {
          buildInputs = [ pkgs.awscli2 ];
	  shellHook = ''
	    export AWS_PROFILE=${awsProfile}
	  '';
        };
      }
    );
}
