{
  inputs = {
    flake-parts.url = "github:hercules-ci/flake-parts";
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = inputs@{ flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      systems = [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
      perSystem = { config, self', inputs', pkgs, system, ... }: {
        packages.default =
        let
          inherit (pkgs) lib rustPlatform;
          inherit (lib.importTOML ./Cargo.toml) package;

          pname = package.name;
          version = package.version;
        in rustPlatform.buildRustPackage {
          inherit pname version;

          src = lib.cleanSource ./.;

          cargoSha256 = "sha256-8Jb1QjJOV2Bnu/TQssdMmaAfWX7Aa2dEnnlkA3sE5MU=";
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            cargo
            rustc
          ];
        };
      };
    };
}
