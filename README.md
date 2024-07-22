# code2nix
This branch became solely for testing and determining why my original script was not quite working correctly. The start of this was when the app would consistenly fail when trying to pull the latest version of my extensions, but succeed when pulling the current versions. I tested with an [upstream script](https://github.com/NixOS/nixpkgs/blob/71770c17e76268cb001cf03ad670a01db8d00ad6/pkgs/applications/editors/vscode/extensions/update_installed_exts.sh) and had the same result. Can't be a bug in my app then right?

I went to debugging and attempting tests, and the next few errors were lost to mindlessly re-running a testing script that behaved **inconsistent** every.. single.. time. Hashes would swap between current <-> latest, it would build sometimes but not other times, switching `curl + nix-hash` and `nix-prefetch-url` made things happen differently (though this one might've been a skill issue). I decided to focus in on `redhat.java`, which would consistently fail on the test for latest versions. First off, trying to pull the `nix-prefetch-url` path resulted in:

#### Current
```console
$ nix-prefetch-url --print-path "https://redhat.gallery.vsassets.io/_apis/public/gallery/publisher/redhat/extension/java/1.33.2024072008/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage"
0java0yqb6yrycsszlv3vj2hapk5xlx3lr02airrkx09cdlwdrw1
/nix/store/gbmx94y50k4fd5iazrizwm8w4valkflh-Microsoft.VisualStudio.Services.VSIXPackage
```

#### Latest
```console
$ nix-prefetch-url --print-path "https://redhat.gallery.vsassets.io/_apis/public/gallery/publisher/redhat/extension/java/latest/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage"
1bf97qy9dvc0d37ligkkw6jh1406awb7mdrzamr5gy9q40jmf2bd
/nix/store/zjikf6maii5b21qiyxh609dqa7h1xhqj-Microsoft.VisualStudio.Services.VSIXPackage
```

How strange, a completely different hash and store path. Obviously this must mean I'm super stupid and these must be two different versions right? Human error back at it again, except..

```console
$ nix run nixpkgs#unzip -- -qc "/nix/store/gbmx94y50k4fd5iazrizwm8w4valkflh-Microsoft.VisualStudio.Services.VSIXPackage" "extension/package.json" | nix run nixpkgs#jq -- -r '.version'
1.33.2024072008
$ nix run nixpkgs#unzip -- -qc "/nix/store/zjikf6maii5b21qiyxh609dqa7h1xhqj-Microsoft.VisualStudio.Services.VSIXPackage" "extension/package.json" | nix run nixpkgs#jq -- -r '.version'
1.33.2024072008
```

What's going on. Why are they internally versioned the same?? It should be noted that nixpkgs **requires** a version, so naturally after downloading the latest one I pulled the version it had in order to (hopefully) guarantee consistency. Of course sometimes no matter what we want, there's evil forces in the world that spread chaos.

Anyways, now that we know there's obviously some weird shit going on, what to do? I immediately `unzip`'d the packages and ran `diff` on them to determine what was going on. This is what I found:

```diff
diff --unified --recursive --text latest/[Content_Types].xml version/[Content_Types].xml
--- latest/[Content_Types].xml  2024-07-20 08:07:48.000000000 +0000
+++ version/[Content_Types].xml 2024-07-20 08:11:08.000000000 +0000
@@ -1,2 +1,2 @@
 <?xml version="1.0" encoding="utf-8"?>
-<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension=".json" ContentType="application/json"/><Default Extension=".vsixmanifest" ContentType="text/xml"/><Default Extension=".ts" ContentType="video/mp2t"/><Default Extension=".md" ContentType="text/markdown"/><Default Extension=".txt" ContentType="text/plain"/><Default Extension=".css" ContentType="text/css"/><Default Extension=".jar" ContentType="application/java-archive"/><Default Extension=".png" ContentType="image/png"/><Default Extension=".xml" ContentType="application/xml"/><Default Extension=".js" ContentType="application/javascript"/><Default Extension=".ini" ContentType="text/plain"/><Default Extension=".py" ContentType="application/octet-stream"/><Default Extension=".bat" ContentType="application/x-msdownload"/><Default Extension=".properties" ContentType="application/octet-stream"/><Default Extension=".dat" ContentType="application/octet-stream"/><Default Extension=".ja" ContentType="application/octet-stream"/><Default Extension=".lib" ContentType="application/octet-stream"/><Default Extension=".cfg" ContentType="application/octet-stream"/><Default Extension=".src" ContentType="application/x-wais-source"/><Default Extension=".bfc" ContentType="application/octet-stream"/><Default Extension=".sym" ContentType="application/octet-stream"/><Default Extension=".h" ContentType="text/x-c"/><Default Extension=".dll" ContentType="application/octet-stream"/><Default Extension=".exe" ContentType="application/octet-stream"/><Default Extension=".policy" ContentType="application/octet-stream"/><Default Extension=".certs" ContentType="application/octet-stream"/><Default Extension=".jfc" ContentType="application/octet-stream"/><Default Extension=".security" ContentType="application/octet-stream"/><Default Extension=".template" ContentType="application/octet-stream"/><Default Extension=".access" ContentType="application/octet-stream"/></Types>
+<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types"><Default Extension=".json" ContentType="application/json"/><Default Extension=".vsixmanifest" ContentType="text/xml"/><Default Extension=".ts" ContentType="video/mp2t"/><Default Extension=".md" ContentType="text/markdown"/><Default Extension=".txt" ContentType="text/plain"/><Default Extension=".css" ContentType="text/css"/><Default Extension=".jar" ContentType="application/java-archive"/><Default Extension=".png" ContentType="image/png"/><Default Extension=".xml" ContentType="application/xml"/><Default Extension=".js" ContentType="application/javascript"/><Default Extension=".ini" ContentType="text/plain"/><Default Extension=".py" ContentType="application/octet-stream"/><Default Extension=".bat" ContentType="application/x-msdownload"/></Types>
Only in latest/extension: jre
diff --unified --recursive --text latest/extension.vsixmanifest version/extension.vsixmanifest
--- latest/extension.vsixmanifest       2024-07-20 08:07:48.000000000 +0000
+++ version/extension.vsixmanifest      2024-07-20 08:11:08.000000000 +0000
@@ -1,7 +1,7 @@
 <?xml version="1.0" encoding="utf-8"?>
        <PackageManifest Version="2.0.0" xmlns="http://schemas.microsoft.com/developer/vsx-schema/2011" xmlns:d="http://schemas.microsoft.com/developer/vsx-schema-design/2011">
                <Metadata>
-                       <Identity Language="en-US" Id="java" Version="1.33.2024072008" Publisher="redhat" TargetPlatform="win32-x64"/>
+                       <Identity Language="en-US" Id="java" Version="1.33.2024072008" Publisher="redhat" />
                        <DisplayName>Language Support for Java(TM) by Red Hat</DisplayName>
                        <Description xml:space="preserve">Java Linting, Intellisense, formatting, refactoring, Maven/Gradle support and more...</Description>
                        <Tags>multi-root ready,keybindings,json,java,__ext_java,__ext_class,java-properties,Java Properties,__ext_properties,gradle-kotlin-dsl,Gradle Kotlin DSL,__ext_gradlekts,linters</Tags>
```

It seems the latest one liked to add a lot of nonsense to its files, more specifically `extensions.vsixmanifest` and `[Content_Types].xml`. There's a **lot** wrong with this, such as the weird assumption latest makes with adding `TargetPlatform="win32-x64"` to the metadata, or why this discrepancy exists at all, but I digress. Without further details, it's fundamentally not really possible to fix this heap of nonsense, especially if it requires changes upstream. Let this be a lesson folks, please don't have inconsistent shit happen, it's a thorn in **everybody's** side, Nix or not.

## License
This repository is subject to the terms of the [ACSL](https://anticapitalist.software/).
