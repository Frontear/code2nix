import hashlib
import json
import sys
import subprocess

from concurrent.futures import ThreadPoolExecutor, as_completed
from io import StringIO, BytesIO
from shutil import which
from urllib.request import urlopen
from zipfile import ZipFile

BIN_VSCODE = which("code")
BIN_HASH = which("nix-hash")

def run_cmd(bin, *args):
    return subprocess.run([ bin ] + list(args), stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True).stdout.splitlines()

def download_ext(ext):
    publisher, _, name_ver = ext.partition(".")
    name, _, version = name_ver.partition("@") # TODO: version acceptable for --show-versions
    url = f"https://{publisher}.gallery.vsassets.io/_apis/public/gallery/publisher/{publisher}/extension/{name}/latest/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage"

    print(f"\033[KDownloading {ext}...", end="\r", file=sys.stderr)

    with urlopen(url) as resp, BytesIO(resp.read()) as fp, ZipFile(fp) as z, z.open("extension/package.json") as f:
        # TODO: run both in parallel
        version = json.load(f)["version"]
        sha256 = run_cmd(BIN_HASH, "--to-base32", "--type", "sha256", hashlib.sha256(fp.read()).hexdigest())[0]

        return ( name, publisher, version, sha256 )

def main():
    with StringIO("") as buff:
        exts = run_cmd(BIN_VSCODE, "--list-extensions")

        buff.write("[\n")

        with ThreadPoolExecutor() as executor:
            for future in as_completed(( executor.submit(download_ext, ext) for ext in exts )):
                name, publisher, version, sha256 = future.result()

                buff.write("  {\n" +
                          f"    name = \"{name}\";\n" +
                          f"    publisher = \"{publisher}\";\n" +
                          f"    version = \"{version}\";\n" +
                          f"    sha256 = \"{sha256}\";\n" +
                           "  }\n")

        buff.write("]")

        print("\r\033[K", end="\r", file=sys.stderr)
        print(buff.getvalue())

if __name__ == "__main__":
    main()