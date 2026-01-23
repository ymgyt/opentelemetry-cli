{
  description = "opentelemetry-cli flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

        opentelemetry-cli = pkgs.rustPlatform.buildRustPackage {
          pname = "opentelemetry-cli";
          version = cargoToml.package.version;

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs =
            with pkgs;
            [
              openssl
            ]
            ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
              pkgs.darwin.apple_sdk.frameworks.Security
              pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
            ];

          meta = with pkgs.lib; {
            description = "OpenTelemetry utility CLI";
            homepage = "https://github.com/ymgyt/opentelemetry-cli";
            license = licenses.mit;
            maintainers = [ ];
          };
        };
      in
      {
        packages.default = opentelemetry-cli;
        packages.opentelemetry-cli = opentelemetry-cli;

        apps.default = {
          type = "app";
          program = "${opentelemetry-cli}/bin/otel";
        };
        apps.opentelemetry-cli = {
          type = "app";
          program = "${opentelemetry-cli}/bin/otel";
        };

        devShells.default = pkgs.mkShell {

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];
        };
      }
    );
}
