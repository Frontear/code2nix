{
  mkShell,

  code2nix,
}:
mkShell {
  inputsFrom = [
    code2nix
  ];
}
