import json
import sys

from concurrent.futures import ThreadPoolExecutor
from io import StringIO, BytesIO
from shutil import which
from subprocess import run
from urllib.request import urlopen
from zipfile import ZipFile

BIN_VSCODE = which("code")
BIN_PREFETCH = which("nix-prefetch-url")

def run_cmd(bin, *args):
    return run([ bin ] + list(args), capture_output=True, text=True).stdout.splitlines()

def parse_ext(args):
    ext, buff = args

    publisher, name = ext.split(".")
    url_vsix = f"https://{publisher}.gallery.vsassets.io/_apis/public/gallery/publisher/{publisher}/extension/{name}/latest/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage"

    print(f"\033[KDownloading {ext}...", end="\r", file=sys.stderr)

    with ZipFile(BytesIO(urlopen(url_vsix).read())) as z:
        with z.open("extension/package.json") as f:
            version = json.load(f)["version"]
            url_hash = f"https://marketplace.visualstudio.com/_apis/public/gallery/publishers/{publisher}/vsextensions/{name}/{version}/vspackage"
            sha256 = run_cmd(BIN_PREFETCH, url_hash)[0]

            # Normally this would be a concurrency issue but the GIL takes care of this for us
            buff.write("  {\n" +
                      f"    name = \"{name}\";\n" +
                      f"    publisher = \"{publisher}\";\n" +
                      f"    version = \"{version}\";\n" +
                      f"    sha256 = \"{sha256}\";\n" +
                       "  }\n")

def main():
    with StringIO("") as buff:
        exts = list(map(lambda x: (x, buff), run_cmd(BIN_VSCODE, "--list-extensions"))) # provides buffer

        buff.write("[\n")

        with ThreadPoolExecutor(max_workers=len(exts)) as executor:
            executor.map(parse_ext, exts)

        buff.write("]")

        print("\033[K", end="\r")
        print(buff.getvalue())

if __name__ == "__main__":
    main()