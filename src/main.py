import json
import os
import sys
import subprocess

from argparse import ArgumentParser
from concurrent.futures import ThreadPoolExecutor, as_completed
from io import StringIO, BytesIO
from shutil import which
from sortedcontainers import SortedDict
from urllib.request import urlopen
from zipfile import ZipFile

BIN_VSCODE = which("code")
BIN_PREFETCH = which("nix-prefetch-url")

def run_cmd(bin, *args):
    return subprocess.run([ bin ] + list(args), capture_output=True, text=True).stdout.splitlines()

def download_ext(ext, current):
    publisher, _, name_ver = ext.partition(".")
    name, _, version = name_ver.partition("@")

    if current:
        url = f"https://{publisher}.gallery.vsassets.io/_apis/public/gallery/publisher/{publisher}/extension/{name}/{version}/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage"
    else:
        url = f"https://{publisher}.gallery.vsassets.io/_apis/public/gallery/publisher/{publisher}/extension/{name}/latest/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage"

    print(f"\033[KDownloading {ext}...", end="\r", file=sys.stderr)

    if current:
        sha256 = run_cmd(BIN_PREFETCH, url)[0]
    else:
        with urlopen(url) as resp, BytesIO(resp.read()) as fp, ZipFile(fp) as z, z.open("extension/package.json") as f:
            # TODO: run both in parallel
            version = json.load(f)["version"]

        # I love having to download something twice. see: https://github.com/Frontear/code2nix/tree/rewrite
        url = f"https://{publisher}.gallery.vsassets.io/_apis/public/gallery/publisher/{publisher}/extension/{name}/{version}/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage"
        sha256 = run_cmd(BIN_PREFETCH, url)[0]

    return ( name, publisher, version, sha256 )

def main():
    parser = ArgumentParser(description="Downloads the latest version of your vscode extensions and outputs them into a nix expression")
    parser.add_argument("workers", default=os.cpu_count(), metavar="N", type=int, help=f"How many extensions to download concurrently (default: `{os.cpu_count()}`)")
    parser.add_argument("strategy", default="current", choices=["latest", "current"], help="Download strategy for downloading extensions, either by current version or by latest found (default: current)")

    args = parser.parse_args()

    ext_dict = SortedDict()
    exts = run_cmd(BIN_VSCODE, "--list-extensions", "--show-versions" if args.strategy == "current" else "")

    with ThreadPoolExecutor(max_workers=args.workers) as executor:
        for future in as_completed(( executor.submit(download_ext, ext, args.strategy == "current") for ext in exts )):
            name, publisher, version, sha256 = future.result()
            ext_dict[name] = (publisher, version, sha256)

    with StringIO("") as buff:
        buff.write("[\n")
        for name, vals in ext_dict.items():
            buff.write("  {\n" +
                      f"    name = \"{name}\";\n" +
                      f"    publisher = \"{vals[0]}\";\n" +
                      f"    version = \"{vals[1]}\";\n" +
                      f"    sha256 = \"{vals[2]}\";\n" +
                       "  }\n")

        buff.write("]")

        print("\r\033[K", end="\r", file=sys.stderr)
        print(buff.getvalue())

if __name__ == "__main__":
    main()
