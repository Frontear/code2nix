import json
import sys
import subprocess

from concurrent.futures import ThreadPoolExecutor
from io import StringIO, BytesIO
from multiprocessing import cpu_count as cores
from shutil import which
from urllib.request import urlopen
from zipfile import ZipFile

BIN_VSCODE = which("code")
BIN_PREFETCH = which("nix-prefetch-url")

def run_cmd(bin, *args):
    return subprocess.run([ bin ] + list(args), capture_output=True, text=True).stdout.splitlines()

def parse_ext(ext):
    publisher, name = ext.split(".")
    url_vsix = f"https://{publisher}.gallery.vsassets.io/_apis/public/gallery/publisher/{publisher}/extension/{name}/latest/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage"

    print(f"\033[KDownloading {ext}...", end="\r", file=sys.stderr)

    with ZipFile(BytesIO(urlopen(url_vsix).read())) as z:
        with z.open("extension/package.json") as f:
            version = json.load(f)["version"]
            url_hash = f"https://marketplace.visualstudio.com/_apis/public/gallery/publishers/{publisher}/vsextensions/{name}/{version}/vspackage"
            sha256 = run_cmd(BIN_PREFETCH, url_hash)

            return {
                "name": name,
                "publisher": publisher,
                "version": version,
                "sha256": sha256[0]
            }

def main():
    exts = run_cmd(BIN_VSCODE, "--list-extensions")
    tasks = []

    with ThreadPoolExecutor(max_workers=cores()) as executor:
        for ext in exts:
            tasks.append(executor.submit(parse_ext, ext))

        with StringIO("") as s:
            s.write("[\n")

            for task in tasks:
                if not task.done():
                    tasks.append(task)
                    continue

                attr = task.result()

                s.write("  {\n")
                s.write(f"    name = \"{attr['name']}\";\n")
                s.write(f"    publisher = \"{attr['publisher']}\";\n")
                s.write(f"    version = \"{attr['version']}\";\n")
                s.write(f"    sha256 = \"{attr['sha256']}\";\n")
                s.write("  }\n")

            s.write("]")

            print("\033[K")
            print(s.getvalue())

if __name__ == "__main__":
    main()