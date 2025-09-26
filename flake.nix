{
  description = "Flake pour lilo-et-stych";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        packageName = "lilo-et-stych";
      in
      {
        packages.${packageName} = pkgs.rustPlatform.buildRustPackage {
          pname = packageName;
          version = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package.version;

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = with pkgs; [
            pkg-config
          ];

          buildInputs = with pkgs; [
            openssl
            sqlite
          ];

          # Désactiver les tests pendant la construction car ils peuvent nécessiter
          # un accès réseau ou des configurations spécifiques
          doCheck = false;

          meta = with pkgs.lib; {
            description = "Un bot Rust pour interagir avec la plateforme Stych.fr et Matrix";
            homepage = "https://github.com/baptistemillou/lilo-et-stych";
            license = licenses.mit;
            maintainers = with maintainers; [ ];
          };
        };

        defaultPackage = self.packages.${system}.${packageName};

        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            rustc
            cargo
            rustfmt
            clippy
            openssl
            sqlite
            pkg-config
          ];

          RUST_BACKTRACE = 1;
        };
      });
}