{ ... }: {
  perSystem = { pkgs, ... }: {
    devShells.default = pkgs.callPackage ./shell.nix { };

    packages.default = pkgs.python3Packages.callPackage ./package.nix { };
  };
}
