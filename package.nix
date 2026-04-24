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
