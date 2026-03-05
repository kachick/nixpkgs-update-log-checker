{
  lib,
  craneLib,
  commonArgs,
  cargoArtifacts,
  versionCheckHook,
}:

let
  mainProgram = "nixpkgs-update-log-checker";
in
craneLib.buildPackage (
  commonArgs
  // {
    inherit cargoArtifacts;
    pname = "nixpkgs-update-log-checker";
    version = with builtins; (fromTOML (readFile ./Cargo.toml)).package.version;

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
  }
)
