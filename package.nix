{
  pkgs ? import <nixpkgs> {
    config.allowUnfree = true;
  }
}:
let
  inherit (pkgs) vscode-utils vscode-with-extensions;
in vscode-with-extensions.override {
  vscodeExtensions = vscode-utils.extensionsFromVscodeMarketplace (import ./current.nix);
  #vscodeExtensions = vscode-utils.extensionsFromVscodeMarketplace (import ./latest.nix);
}
