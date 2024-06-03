# vsc2nix
A simple wrapper script that uses [nix4vscode](https://github.com/nix-community/nix4vscode) to generate a nix expression of your vscode extensions.

## How to use

At the moment this script is hacked together really badly, mostly in an effort to have more consistent declarations with my version of vscode.

- Create an `extensions.txt` file with `code --list-extensions > extensions.txt`
- Run the python script to write a `config.toml` file
- Execute `nix run . config.toml > extensions.nix`
- Import the resulting nix expression to your configuration

## License

My shit is GNU GPL v3. Everything else is subject to their own. Formal statement will be here soon

<!--
Notes: :D
ripunzip is too heavy honestly. Want to switch to diff crates from it.
-->