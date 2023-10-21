{
  description = "opentelemetry-cli flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
      url = "github:ipetkov/crane";
      inputs = { nixpkgs.follows = "nixpkgs"; };
    };

    flake-utils.url = "github:numtide/flake-utils";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = {
        nixpkgs.follows = "nixpkgs";
        flake-utils.follows = "flake-utils";
      };
    };
  };

  outputs = { self, nixpkgs, flake-utils, crane, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default;

        craneLib = (crane.mkLib pkgs).overrideToolchain rustToolchain;

        # Tell crane Cargo.toml path to inspect package name and version
        crate = craneLib.crateNameFromCargoToml { cargoToml = ./Cargo.toml; };

        opentelemetry-cli = craneLib.buildPackage {
          inherit (crate) pname version;
          src = craneLib.cleanCargoSource (craneLib.path ./.);

          buildInputs = [ ] ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [ ];
        };
      in {
        packages.default = self.packages."${system}".opentelemetry-cli;
        packages.opentelemetry-cli = opentelemetry-cli;
        apps.default = {
          type = "app";
          program = "${opentelemetry-cli}/bin/otel";
        };
        apps.opentelemetry-cli = {
          type = "app";
          program = "${opentelemetry-cli}/bin/otel";
        };
        devShells.default = pkgs.mkShell { buildInputs = [ pkgs.nixfmt ]; };
      });
}
