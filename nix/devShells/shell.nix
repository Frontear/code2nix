{
  mkShell,

  code2nix,

  vscode-with-extensions,
  vscode-utils,
}:
mkShell {
  inputsFrom = [
    code2nix
  ];

  packages = [
    (vscode-with-extensions.override {
      vscodeExtensions = vscode-utils.extensionsFromVscodeMarketplace (
        import ./extensions.nix
      );
    })
  ];
}
