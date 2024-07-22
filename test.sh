#!/usr/bin/env nix-shell
#! nix-shell -i bash -p curl jq nix unzip 

function extension() {
    local pub="$1"
    local name="$2"
    local ver="${3:-latest}"
    local hash=""

    local tmp="$(mktemp -d)"
    local file="$tmp/extension.zip"

    curl --silent -o "$file" "https://${pub}.gallery.vsassets.io/_apis/public/gallery/publisher/${pub}/extension/${name}/${ver}/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage"
    
    if [ "$ver" = "latest" ]; then
        local ver=$(jq -r '.version' <(unzip -qc "$file" "extension/package.json"))
    fi

    local hash=$(nix-hash --flat --base32 --type sha256 "$file" 2> /dev/null)

    rm -rf "$tmp"

    cat <<- EOF
  {
    name = "${name}";
    publisher = "${pub}";
    version = "${ver}";
    sha256 = "${hash}";
  }
EOF
}

function generate_expressions() {
    local type="$1" # One of: latest, current
    local ext_nix="${type}/extensions.nix"
    local pkg_nix="${type}/package.nix"

    local args="--list-extensions"

    if [ "$type" = "current" ]; then
        args+=" --show-versions"
    fi

    mkdir -p "${type}"

    echo "[" > "$ext_nix"

    for i in $(code $args); do
        if [ "$type" = "latest" ]; then
            extension "$(echo "$i" | cut -d@ -f1 | cut -d. -f1)" "$(echo "$i" | cut -d@ -f1 | cut -d. -f2)" >> "$ext_nix"
        else
            extension "$(echo "$i" | cut -d@ -f1 | cut -d. -f1)" "$(echo "$i" | cut -d@ -f1 | cut -d. -f2)" "$(echo "$i" | cut -d@ -f2)" >> "$ext_nix"
        fi
    done
    echo "]" >> "$ext_nix"

    cat << EOF > "$pkg_nix"
{
  pkgs ? import <nixpkgs> { config.allowUnfree = true; }
}:
pkgs.vscode-with-extensions.override {
  vscodeExtensions = pkgs.vscode-utils.extensionsFromVscodeMarketplace (import ./$(basename "$ext_nix");
}
EOF

    echo "$pkg_nix"
}

if ! which code; then
    echo "VSCode not found in PATH, cannot continue"
    exit 1
fi

f1=$(generate_expressions "current")
f2=$(generate_expressions "latest")

nix-build --quiet --no-out-link "$f1"
nix-build --quiet --no-out-link "$f2"
