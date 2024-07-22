{
  pkgs ? import <nixpkgs> { config.allowUnfree = true; }
}:
pkgs.vscode-with-extensions.override {
vscodeExtensions = pkgs.vscode-utils.extensionsFromVscodeMarketplace (import ./extensions.nix);
}
