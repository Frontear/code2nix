{
  ...
}:
{
  perSystem = { pkgs, ... }: {
    packages.default = pkgs.python3Packages.callPackage ./package.nix {};
  };
}
