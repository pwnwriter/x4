{
  description = "Run commands to a server via ssh";

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
          name = "sshy";
          packages =
            (with pkgs; [
              cmake
            ])
            ++ (pkgs.lib.optional pkgs.stdenvNoCC.isDarwin (darwinDeps pkgs));
        };
      });

      formatter = forAllSystems (pkgs: pkgs.nixfmt-rfc-style);
      packages = forAllSystems (pkgs: {
        sshy =
          with pkgs;
          let
            fs = lib.fileset;
            sourceFiles = fs.unions [
              ./Cargo.lock
              ./Cargo.toml
              ./src
            ];

            pname = cargoToml.package.name;
            version = cargoToml.package.version;
            cargoLock.lockFile = ./Cargo.lock;
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
              git
            ];
            buildInputs = lib.optionals stdenv.isDarwin darwinBuildInputs;
            preBuildPhases = [ "cargoFmt" ];
            cargoFmt = ''
              cargo fmt --manifest-path ./Cargo.toml --all --check
            '';
            preInstallPhases = [ "clippy" ];
            clippy = ''
              cargo clippy -- --deny warnings
            '';
          };
        default = self.packages.${pkgs.system}.sshy;
      });
      apps = forAllSystems (pkgs: {
        default = {
          type = "app";
          program = "${self.packages.${pkgs.system}.sshy}/bin/sshy";
        };
      });
    };
}
