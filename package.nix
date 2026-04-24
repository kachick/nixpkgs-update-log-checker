{
  lib,
  innerBuild,
  versionCheckHook,
}:

let
  mainProgram = "nixpkgs-update-log-checker";
in
innerBuild.overrideAttrs (oldAttrs: {
  inherit mainProgram;

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
      lib.maintainers.kachick
    ];
  };
})
