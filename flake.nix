{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    nix4vscode = {
      url = "github:nix-community/nix4vscode";
      flake = false;
    };
  };

  outputs = { self, ... } @ inputs:
  let
    # https://ayats.org/blog/no-flake-utils
    eachSystem = function: inputs.nixpkgs.lib.genAttrs [
      "x86_64-linux"
      "x86_64-darwin"
      "aarch64-linux"
      "aarch64-darwin"
    ] (system: function (import inputs.nixpkgs {
      inherit system;
      config.allowUnfree = true;
    }));
  in {
    packages = eachSystem (pkgs: {
      default = pkgs.callPackage ({ rustPlatform }: rustPlatform.buildRustPackage rec {
        pname = "nix4vscode";
        version = "0.1.0";

        src = inputs.nix4vscode;

        cargoSha256 = "sha256-9BM0cpHvri9cACbQX++OmWr51fBQenJabNewszgwGDs=";
      }) {};
    });

    devShells = eachSystem (pkgs: {
      default = pkgs.mkShell {
        packages = with pkgs; [
          python3
        ];
      };
    });
  };
}
