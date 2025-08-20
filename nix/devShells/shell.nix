{
  mkShell,

  code2nix,

  vscode-extensions,
  vscode-utils,
  vscode-with-extensions,
}:
let
  vscodeExtensions = [
    vscode-extensions.rust-lang.rust-analyzer
  ] ++ vscode-utils.extensionsFromVscodeMarketplace (import ./extensions.nix);
in mkShell {
  inputsFrom = [
    code2nix
  ];

  packages = [
    (vscode-with-extensions.override {
      inherit vscodeExtensions;
    })
  ];
}
