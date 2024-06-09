import json
import sys
import shutil
import subprocess

from io import BytesIO
from urllib.request import urlopen
from zipfile import ZipFile

if __name__ == "__main__":
    code_bin = shutil.which("code")
    prefetch_bin = shutil.which("nix-prefetch-url")

    extensions = subprocess.run([ code_bin, "--list-extensions" ], capture_output=True, text=True).stdout.splitlines()

    ext_list = []

    for n, ext in enumerate(extensions):
        pub, name = ext.split(".")

        # https://stackoverflow.com/a/31417845/9091276
        print(f"\033[KDownloading {ext}... ({n}/{len(extensions)})", end="\r", file=sys.stderr)

        with ZipFile(BytesIO(urlopen(f"https://{pub}.gallery.vsassets.io/_apis/public/gallery/publisher/{pub}/extension/{name}/latest/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage").read())) as z:
            with z.open("extension/package.json") as f:
                ver = json.load(f)["version"]
                sha256 = subprocess.run([ prefetch_bin, f"https://marketplace.visualstudio.com/_apis/public/gallery/publishers/{pub}/vsextensions/{name}/{ver}/vspackage" ], capture_output=True, text=True).stdout.splitlines()[0]

                ext_list.append({
                    "name": name,
                    "publisher": pub,
                    "version": ver,
                    "sha256": sha256
                })

    print(json.dumps(ext_list))