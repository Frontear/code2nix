{
  lib,
  buildPythonApplication,
  setuptools,
}:
buildPythonApplication {
  pname = "code2nix";
  version = "0.1.0";

  src = lib.cleanSource ../.;

  pyproject = true;
  build-system = [
    setuptools
  ];

  meta = with lib; {
    description = "A simple python script which downloads the latest versions of your currently installed vscode extensions and outputs a nix expression wrapping `extensionsFromMarketplace`";
    homepage = "https://github.com/Frontear/code2nix";
    license = licenses.gpl3;
    maintainers = with maintainers; [ frontear ];
  };
}
