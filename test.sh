#!/usr/bin/env nix-shell
#! nix-shell -i bash -p curl jq nix unzip 

name="remote-ssh-edit"
pub="ms-vscode-remote"
ver="0.47.2"
sha=""

# Current
curl --silent -o "/tmp/current.zip" "https://${pub}.gallery.vsassets.io/_apis/public/gallery/publisher/${pub}/extension/${name}/${ver}/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage"

PREFETCH_HASH=$(nix-prefetch-url "https://marketplace.visualstudio.com/_apis/public/gallery/publishers/${pub}/vsextensions/${name}/${ver}/vspackage" 2> /dev/null)
NIX_HASH=$(nix-hash --flat --base32 --type sha256 "/tmp/current.zip" 2> /dev/null)

echo "nix-prefetch-url: $PREFETCH_HASH"
echo "nix-hash: $NIX_HASH"

if [ "$PREFETCH_HASH" != "$NIX_HASH" ]; then
    echo "Error: nix-prefetch-url and nix-hash are resulting in different hashes" >> /dev/stderr
fi

# Latest

curl --silent -o "/tmp/latest.zip" "https://${pub}.gallery.vsassets.io/_apis/public/gallery/publisher/${pub}/extension/${name}/latest/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage"
ver=$(jq -r '.version' <(unzip -qc "/tmp/latest.zip" "extension/package.json"))

PREFETCH_HASH=$(nix-prefetch-url "https://marketplace.visualstudio.com/_apis/public/gallery/publishers/${pub}/vsextensions/${name}/${ver}/vspackage" 2> /dev/null)
NIX_HASH=$(nix-hash --flat --base32 --type sha256 "/tmp/latest.zip" 2> /dev/null)

echo "nix-prefetch-url: $PREFETCH_HASH"
echo "nix-hash: $NIX_HASH"

if [ "$PREFETCH_HASH" != "$NIX_HASH" ]; then
    echo "Error: nix-prefetch-url and nix-hash are resulting in different hashes" >> /dev/stderr
fi
