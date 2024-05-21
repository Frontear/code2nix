import subprocess, tomllib

extension_list = []

if __name__ == "__main__":
    with open("extensions.txt", "r") as f:
        extension_list = [ x.strip() for x in f.readlines() ]

    with open("config.toml", "w") as f:
        f.write("vscode_version = \"1.88.1\"")
        f.write("\n")

        for ext in extension_list:
            pubn, extn = ext.split(".")
            f.write("\n")
            f.write("[[extensions]]\n")
            f.write(f"publisher_name = \"{pubn}\"\n")
            f.write(f"extension_name = \"{extn}\"\n")
