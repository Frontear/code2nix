import hashlib
import json
import sys
import subprocess

from concurrent.futures import ThreadPoolExecutor
from io import StringIO, BytesIO
from multiprocessing import cpu_count
from shutil import which
from urllib.request import urlopen
from zipfile import ZipFile

BIN_VSCODE = which("code")
BIN_HASH = which("nix-hash")

def run_cmd(bin, *args):
    return subprocess.run([ bin ] + list(args), stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True).stdout.splitlines()

def parse_ext(args):
    ext, buff = args

    publisher, name = ext.split(".")
    url_vsix = f"https://{publisher}.gallery.vsassets.io/_apis/public/gallery/publisher/{publisher}/extension/{name}/latest/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage"

    print(f"\033[KDownloading {ext}...", end="\r", file=sys.stderr)

    with urlopen(url_vsix) as resp, BytesIO(resp.read()) as fp, ZipFile(fp) as z, z.open("extension/package.json") as f:
        version = json.load(f)["version"]
        sha256 = run_cmd(BIN_HASH, "--to-base32", "--type", "sha256", hashlib.sha256(fp.read()).hexdigest())[0]

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

        with ThreadPoolExecutor(max_workers=cpu_count() * 2) as executor:
            executor.map(parse_ext, exts)

        buff.write("]")

        print("\r\033[K", end="\r", file=sys.stderr)
        print(buff.getvalue())

if __name__ == "__main__":
    main()