{
  lib,
  rustPlatform,

  pkg-config,
  openssl,
}:
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "code2nix";
  version = "0.1.0";

  src = with lib.fileset; toSource {
    root = ../../.;
    fileset = unions [
      ../../src
      ../../Cargo.toml
      ../../Cargo.lock
    ];
  };

  cargoLock.lockFile = ../../Cargo.lock;

  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = [
    openssl
  ];

  meta = with lib; {
    homepage = "https://github.com/Frontear/code2nix";
    license = licenses.gpl3;
    maintainers = with maintainers; [ frontear ];

    mainProgram = "code2nix";
  };
})
