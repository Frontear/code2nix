# code2nix
A simple python script which downloads the latest versions of your currently installed vscode extensions and retrieves their metadata for declarative extensions via `extensionsFromVscodeMarketplace`.

Some future implementation details include:
- [ ] auto-updating produced expression
- [x] removing the json middleman (python -> nix)
- [x] parallelization (speeding up processing and downloads)
- [ ] version checking for compatibility
- [ ] reducing memory and network i/o requirements

## How to use?
The syntax for the command follows `main.py N {latest,current}`.

- **N** (default `nproc`): no. of concurrent extension downloads at a time
- **{latest,current}** (default `"current"`): whether to consider the currently installed version of an extension or simply pull from latest

> [!NOTE]
> Due to unfixable issues on my end, please note that trying to pull the latest versions will almost always be 2x longer.
> This is due to the fact that I have to download the vsix packages twice in order to guarantee consistency.
> See [the "rewrite" branch](https://github.com/Frontear/code2nix/tree/rewrite) for more details.

```console
$ python src/main.py $(nproc) latest > latest.nix # the latest versions of your extensions
$ python src/main.py $(nproc) current > current.nix # the current versions of your extensions
```

Import the file created from the above commands into your configurations.
<details>
    <summary>NixOS Configuration</summary>
    <br />
    <div class="highlight highlight-source-nix">
        <pre>
# configuration.nix
{ config, lib, pkgs, ... }: {
    environment.systemPackages = with pkgs; [
        (vscode-with-extensions.override {
           vscodeExtensions = pkgs.vscode-utils.extensionsFromVscodeMarketplace (import ./ext.nix);
        })
    ];
}
        </pre>
    </div>
</details>
<details>
    <summary>Home Manager Configuration</summary>
    <br />
    <div class="highlight highlight-source-nix">
        <pre>
# home.nix
{ config, lib, pkgs, ... }: {
    programs.vscode.enable = true;
    programs.vscode.extensions = pkgs.vscode-utils.extensionsFromVscodeMarketplace (import ./ext.nix);
}
        </pre>
    </div>
</details>


## License
All code in this project is licensed under the GNU GPL v3 license. Extensions remained licensed under their original license and any other licenses that pertain to them (this license excluded).
