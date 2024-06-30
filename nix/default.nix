{ ... }: {
  perSystem = { pkgs, ... }: {
    devShells.default = pkgs.callPackage ./shell.nix { };

    packages.default = pkgs.callPackage ./package.nix { };
  };
}
