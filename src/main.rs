use std::{env, str};
use std::process::Command;

use std::fs::File;
use std::io::{BufReader, Cursor};

use semver::{Version};

use reqwest::blocking::Client;

// "https://$1.gallery.vsassets.io/_apis/public/gallery/publisher/$1/extension/$2/latest/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage"

fn main() {
    // Get VSCode version
    let ver_cmd = Command::new("code").arg("--version").output().expect("Failed to execute 'code --version'");
    let mut ver_out = str::from_utf8(&ver_cmd.stdout).expect("Failed to convert 'code --version' stdout to str").lines();
    let version = Version::parse(ver_out.next().expect("Failed to read version line from 'code --version' iterator")).expect("Failed to parse 'code --version' str into semver::Version");

    // Get VSCode extensions
    let ext_cmd = Command::new("code").arg("--list-extensions").output().expect("Failed to execute 'code --list-extensions'");
    let mut ext_out = str::from_utf8(&ext_cmd.stdout).expect("Failed to convert 'code --list-extensions' stdout to str").lines();

    let mut ext_dir = env::temp_dir();
    ext_dir.push("code2nix");
    ext_dir.push("extension"); // guaranteed from unzipping

    let client = Client::new();

    while let Some(line) = ext_out.next() {
        let split: Vec<&str> = line.split(".").collect();
        let url = format!("https://{0}.gallery.vsassets.io/_apis/public/gallery/publisher/{0}/extension/{1}/latest/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage", split[0], split[1]);

        let bytes = client.get(url).send().unwrap().bytes().unwrap();
        zip_extract::extract(Cursor::new(bytes), &ext_dir.parent().unwrap(), true).unwrap();

        let json = serde_json::from_reader(BufReader::new(File::open(ext_dir.join("package.json")).unwrap())).unwrap();
        println!("{:#?}", json);

        break;
    }
}