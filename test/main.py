from collections import defaultdict
from concurrent.futures import ThreadPoolExecutor
from os import cpu_count
from shutil import which
from subprocess import run
from time import time, sleep
from urllib.request import urlopen

def clear():
    print("\033c", end="")

def download_ext(ext):
    publisher, _, name = ext.partition(".")
    url = f"https://{publisher}.gallery.vsassets.io/_apis/public/gallery/publisher/{publisher}/extension/{name}/latest/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage"

    start = time()

    with urlopen(url) as resp:
        print(f"\033[K{ext} (size: {len(resp.read())}) took {time() - start:.2f}s", end="\r")

if __name__ == "__main__":
    benchmarks = defaultdict(list)
    extensions = run([ which("code"), "--list-extensions" ], capture_output=True, text=True).stdout.splitlines()

    try:
        # test multiple workers
        for i in range(1, min(32, cpu_count() * 2)):
            # benchmark a min of 10 times to obtain good results (hopefully)
            for x in range(10):
                clear()
                print(f"\033[KTesting (workers: {i}, run {x + 1})...", end="\n")
                start = time()

                with ThreadPoolExecutor(max_workers=i) as executor:
                    for ext in extensions:
                        executor.submit(download_ext, ext)

                benchmarks[i].append(time() - start)
                print(f"\033[KTook {benchmarks[i][-1]:.2f}s, sleeping..", end="\r")
                sleep(10) # don't overload hardware
            sleep(10)
    except KeyboardInterrupt:
        pass
    finally: # force always so we don't lose valuable benchmark data
        clear()
        for workers, time in benchmarks.items():
            avg = sum(time) / len(time)
            print(f"at {workers} worker{'s' if workers > 1 else ''}: {avg:.2f}s average")
            print(f"- {', '.join([f'{x:.2f}s' for x in time])}")