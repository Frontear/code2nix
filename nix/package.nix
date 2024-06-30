{
  lib,
  python3,
  writeShellApplication,
}:
let
  src = lib.cleanSource ../src;
in writeShellApplication {
  name = "code2nix";

  runtimeInputs = [
    python3
  ];

  text = ''
    python ${src}/main.py "$@"
  '';

  meta = with lib; {
    description = "A simple python script which downloads the latest versions of your currently installed vscode extensions and outputs a nix expression wrapping `extensionsFromMarketplace`";
    homepage = "https://github.com/Frontear/code2nix";
    license = licenses.gpl3;
    maintainers = with maintainers; [ frontear ];
  };
}
