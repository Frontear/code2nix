{
  mkShellNoCC,

  code2nix,
}:
mkShellNoCC {
  inputsFrom = [
    code2nix
  ];
}
