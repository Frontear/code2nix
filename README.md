# code2nix
A simple python script which downloads the latest versions of your currently installed vscode extensions and retrieves their metadata for declarative extensions via `extensionsFromVscodeMarketplace`.

Some future implementation details include:
- [ ] auto-updating produced expression
- [x] removing the json middleman (python -> nix)
- [ ] parallelization (speeding up processing and downloads)

## How to use?
Generate the extensions json using `python src/main.py > ext.nix`, then attach it for your extensions as `pkgs.vscode-utils.extensionsFromVscodeMarketplace (import ./ext.nix)`.
This may look like the following:
```nix
# configuration.nix
{ config, lib, pkgs, ... }: {
    environment.systemPackages = with pkgs; [
        (vscode-with-extensions.override {
           vscodeExtensions = pkgs.vscode-utils.extensionsFromVscodeMarketplace (import ./ext.nix);
        })
    ];
}
```

## License
All code in this project is licensed under the GNU GPL v3 license.