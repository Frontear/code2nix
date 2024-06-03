{
  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = inputs@{ flake-parts, ... }: flake-parts.lib.mkFlake { inherit inputs; } {
    systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
    perSystem = { config, self', inputs', pkgs, system, ... }: {
      packages.default = pkgs.callPackage ({ lib, rustPlatform }: rustPlatform.buildRustPackage {
        inherit ((lib.importTOML ./Cargo.toml).package) name version;

        src = lib.cleanSource ./.;

        nativeBuildInputs = with pkgs; [
          pkg-config
        ];

        buildInputs = with pkgs; [
          openssl
        ];

        cargoSha256 = "sha256-qDoSq8L4swOi7jXLCOTaCYbp6CDXNeDQRuo9oRMC67E=";
      }) {};

      devShells.default = pkgs.mkShell {
        inputsFrom = [
          self'.packages.default
        ];
      };
    };
  };
}
