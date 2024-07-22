#!/usr/bin/env nix-shell
#! nix-shell -i bash -p curl jq nix unzip 

name="remote-ssh-edit"
pub="ms-vscode-remote"
ver="0.47.2"
sha=""

function get_current() {
    local pub="$1"
    local name="$2"
    local ver="$3"
    local hash=""

    local hash=$(nix-prefetch-url "https://marketplace.visualstudio.com/_apis/public/gallery/publishers/${pub}/vsextensions/${name}/${ver}/vspackage" 2> /dev/null)

    cat <<- EOF
  {
      name = "${name}";
      publisher = "${pub}";
      version = "${ver}";
      sha256 = "${hash}";
  }
EOF
}

function get_latest() {
    local pub="$1"
    local name="$2"
    local ver=""
    local hash=""

    local tmp=$(mktemp -d)
    local file="$tmp/latest.zip"

    curl --silent -o "$file" "https://${pub}.gallery.vsassets.io/_apis/public/gallery/publisher/${pub}/extension/${name}/latest/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage"
    
    ver=$(jq -r '.version' <(unzip -qc "$file" "extension/package.json"))
    hash=$(nix-hash --flat --base32 --type sha256 "$file" 2> /dev/null)

    rm -rf "$tmp"

    cat <<-EOF
  {
      name = "${name}";
      publisher = "${pub}";
      version = "${ver}";
      sha256 = "${hash}";
  }
EOF
}

echo "["

#for i in $(code --list-extensions); do
for i in $(code --list-extensions --show-versions); do
    VER=$(echo "$i" | cut -d@ -f2)
    NAME=$(echo "$i" | cut -d@ -f1 | cut -d. -f1)
    PUB=$(echo "$i" | cut -d@ -f1 | cut -d. -f2)

    get_current "$NAME" "$PUB" "$VER"
    #get_latest "$NAME" "$PUB"
done

echo "]"
