{
  ...
}:
{
  perSystem = { self', pkgs, ... }: {
    devShells.default = pkgs.callPackage ./shell.nix {
      code2nix = self'.packages.default;
    };
  };
}
