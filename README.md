# code2nix
A simple python script which downloads the latest versions of your currently installed vscode extensions and retrieves their metadata for declarative extensions via `extensionsFromVscodeMarketplace`.

Some future implementation details include:
- [ ] auto-updating produced expression
- [ ] removing the json middleman (python -> nix)
- [ ] parallelization (speeding up processing and downloads)

## How to use?
1. Generate the extensions json using `python src/main.py > ext.json`
2. Convert it into a nix expression via `nix eval --impure --expr 'builtins.fromJSON (builtins.readFile ./ext.json)' > ext.nix`
3. *optional*: Format the resulting expression with `nix run nixpkgs#nixfmt -- ext.nix`
4. Within your configuration, copy this file over and attach it for your extensions as `pkgs.vscode-utils.extensionsFromVscodeMarketplace (import ./ext.nix)`, this may look like the following:
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