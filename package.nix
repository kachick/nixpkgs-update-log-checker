{
  lib,
  versionCheckHook,
}:

let
  mainProgram = "nixpkgs-update-log-checker";
in
{
  nixpkgs-update-log-checker = oldAttrs: {
    inherit mainProgram;

    src = lib.fileset.toSource {
      root = ./.;
      fileset = lib.fileset.unions [
        ./src
        ./Cargo.toml
        ./Cargo.lock
      ];
    };

    nativeInstallCheckInputs = (oldAttrs.nativeInstallCheckInputs or [ ]) ++ [
      versionCheckHook
    ];
    doInstallCheck = true;
    versionCheckProgram = "${placeholder "out"}/bin/${mainProgram}";
    versionCheckProgramArg = "--version";

    meta = (oldAttrs.meta or { }) // {
      inherit mainProgram;
      description = "CLI to check the update log of nixpkgs";
      homepage = "https://github.com/kachick/nixpkgs-update-log-checker";
      license = lib.licenses.mit;
      maintainers = with lib.maintainers; [
        kachick
      ];
    };
  };
}
