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
      maintainers = with lib.maintainers; [
        kachick
      ];
    };
  };
}
