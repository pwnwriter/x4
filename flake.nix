{
  description = "Execute shell commands to a srver via ssh protocol";

  inputs.nixpkgs.url = "github:nixos/nixpkgs/nixpkgs-unstable";

  outputs =
    { self, nixpkgs, ... }:
    let
      forAllSystems =
        function:
        nixpkgs.lib.genAttrs [
          "x86_64-linux"
          "aarch64-linux"
          "x86_64-darwin"
          "aarch64-darwin"
        ] (system: function nixpkgs.legacyPackages.${system});

      darwinDeps =
        pkgs:
        with pkgs;
        with pkgs.darwin.apple_sdk.frameworks;
        [
          Cocoa
          libiconv
        ];

      cargoToml = with builtins; (fromTOML (readFile ./Cargo.toml));
    in
    {
      devShells = forAllSystems (pkgs: {
        default = pkgs.mkShell {
          name = "dev-shell";
          packages =
            (with pkgs; [
              cmake
              git-cliff
            ])
            ++ (pkgs.lib.optional pkgs.stdenv.isDarwin (darwinDeps pkgs));

          shellHook = ''
            export PATH="$PATH:$(pwd)/target/debug"
          '';

        };
      });

      formatter = forAllSystems (pkgs: pkgs.nixfmt-rfc-style);

      packages = forAllSystems (pkgs: {
        x4 =
          with pkgs;
          let
            fs = lib.fileset;
            sourceFiles = fs.unions [
              ./Cargo.lock
              ./Cargo.toml
              ./src
              ./examples
            ];

            cargoLock.lockFile = ./Cargo.lock;
            pname = cargoToml.package.name;
            version = cargoToml.package.version;

            darwinBuildInputs = (darwinDeps pkgs);
          in

          pkgs.rustPlatform.buildRustPackage {

            inherit pname version cargoLock;

            src = fs.toSource {
              root = ./.;
              fileset = sourceFiles;
            };

            nativeBuildInputs = [
              clippy
              rustfmt
              openssl
              pkg-config
              cmake
              perl
            ];

            buildInputs = lib.optionals stdenv.isDarwin darwinBuildInputs;

            cargoFmt = ''
              cargo fmt --manifest-path ./Cargo.toml --all --check
            '';

            clippy = ''
              cargo clippy -- --deny warnings
            '';

            preBuildPhases = [ "cargoFmt" ];

            preInstallPhases = [ "clippy" ];

          };

        default = self.packages.${pkgs.system}.x4;

      });

      apps = forAllSystems (pkgs: {
        default = {
          type = "app";
          program = "${self.packages.${pkgs.system}.x4}/bin/x4";
        };
      });
    };
}
