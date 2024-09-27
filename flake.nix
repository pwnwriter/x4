{
  description = "Execute shell command, (down/up)load files to a server via ssh protocol";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs =
    { self, nixpkgs }:
    let
      supportedSystems = [
        "x86_64-linux"
        "aarch64-linux"
        "x86_64-darwin"
        "aarch64-darwin"
      ];

      forAllSystems = nixpkgs.lib.genAttrs supportedSystems;

      darwinDeps =
        pkgs: with pkgs; [
          darwin.apple_sdk.frameworks.Cocoa
          libiconv
        ];

      cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);

    in
    {
      packages = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        {
          sshy =
            let
              pname = cargoToml.package.name;
              version = cargoToml.package.version;
              cargoLock = {
                lockFile = ./Cargo.lock;
              };
            in
            pkgs.rustPlatform.buildRustPackage {
              inherit pname version cargoLock;

              # Use the current source directory
              src = ./.;

              nativeBuildInputs = with pkgs; [
                clippy
                rustfmt
                openssl
                pkg-config
                cmake
                perl
              ];

              buildInputs = pkgs.lib.optional (pkgs.stdenv.isDarwin) (darwinDeps pkgs);

              # Build phases
              buildPhase = ''
                cargo build --release --frozen --locked
              '';

              checkPhase = ''
                cargo test --verbose
              '';

              installPhase = ''
                mkdir -p $out/bin
                cp target/release/sshy $out/bin/
              '';

            };
        }
      );

      devShells = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        {
          default = pkgs.mkShell {
            name = "dev-shell";
            packages = [
              pkgs.cmake
            ] ++ (pkgs.lib.optional pkgs.stdenvNoCC.isDarwin (darwinDeps pkgs));

            shellHook = ''
              export PATH="$PATH:target/debug"
            '';
          };
        }
      );

      apps = forAllSystems (
        system:
        let
          pkgs = import nixpkgs { inherit system; };
        in
        {
          default = {
            type = "app";
            program = "${self.packages.${system}.sshy}/bin/sshy";
          };
        }
      );
    };
}
