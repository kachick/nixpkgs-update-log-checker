{
  lib,
  rustPlatform,
  versionCheckHook,
}:

let
  mainProgram = "nixpkgs-update-log-checker";
in
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "nixpkgs-update-log-checker";
  version = with builtins; (fromTOML (readFile ./Cargo.toml)).package.version;

  src = lib.fileset.toSource {
    root = ./.;
    fileset = lib.fileset.unions [
      ./src
      ./Cargo.toml
      ./Cargo.lock
    ];
  };

  cargoLock.lockFile = ./Cargo.lock;

  nativeInstallCheckInputs = [
    versionCheckHook
  ];
  doInstallCheck = true;
  versionCheckProgram = "${placeholder "out"}/bin/${mainProgram}";
  versionCheckProgramArg = "--version";

  meta = {
    inherit mainProgram;
    description = "CLI to check the update log of nixpkgs";
    homepage = "https://github.com/kachick/nixpkgs-update-log-checker";
    license = lib.licenses.mit;
    maintainers = with lib.maintainers; [
      kachick
    ];
  };
})
