{
  nixConfig = {
    extra-substituters = [
      "https://cache.garnix.io"
    ];
    extra-trusted-public-keys = [
      "cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g="
    ];
  };

  inputs = {
    nixpkgs.url = "https://channels.nixos.org/nixpkgs-unstable/nixexprs.tar.xz";
    crane.url = "github:ipetkov/crane";
  };

  outputs =
    {
      nixpkgs,
      crane,
      ...
    }:
    let
      lib = nixpkgs.lib;
      forAllSystems = lib.genAttrs lib.systems.flakeExposed;

      mkCraneLib = pkgs: crane.mkLib pkgs;

      mkCommonArgs =
        pkgs:
        let
          craneLib = mkCraneLib pkgs;
        in
        {
          src = craneLib.cleanCargoSource ./.;
          strictDeps = true;

          buildInputs =
            [
              # Add any runtime dependencies here
            ]
            ++ lib.optionals pkgs.stdenv.hostPlatform.isDarwin [
              # Additional darwin specific inputs can be added here
            ];
        };

      mkArtifacts =
        pkgs:
        let
          craneLib = mkCraneLib pkgs;
          commonArgs = mkCommonArgs pkgs;
        in
        craneLib.buildDepsOnly commonArgs;

      mkPackage =
        pkgs:
        let
          craneLib = mkCraneLib pkgs;
          commonArgs = mkCommonArgs pkgs;
          cargoArtifacts = mkArtifacts pkgs;
        in
        pkgs.callPackage ./package.nix {
          inherit craneLib cargoArtifacts commonArgs;
        };
    in
    {
      formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.nixfmt-tree);

      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        rec {
          nixpkgs-update-log-checker = mkPackage pkgs;
          default = nixpkgs-update-log-checker;
        }
      );

      checks = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
          craneLib = mkCraneLib pkgs;
          commonArgs = mkCommonArgs pkgs;
          cargoArtifacts = mkArtifacts pkgs;
          nixpkgs-update-log-checker = mkPackage pkgs;
        in
        {
          inherit nixpkgs-update-log-checker;
          # Build the dependencies once and then reuse them for each check
          nixpkgs-update-log-checker-clippy = craneLib.cargoClippy (
            commonArgs
            // {
              inherit cargoArtifacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            }
          );

          nixpkgs-update-log-checker-fmt = craneLib.cargoFmt (commonArgs // { inherit cargoArtifacts; });

          nixpkgs-update-log-checker-nextest = craneLib.cargoNextest (
            commonArgs // { inherit cargoArtifacts; }
          );
        }
      );

      devShells = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          default = pkgs.mkShell {
            env = {
              # Fix nixd pkgs versions in the inlay hints
              NIX_PATH = "nixpkgs=${pkgs.path}";

              # Workaround for rust-analyzer error: "ERROR can't load standard library, try installing `rust-src`"
              RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
            };

            buildInputs = (
              with pkgs;
              [
                bashInteractive
                nixfmt
                nixd

                rustc
                cargo
                rustfmt
                rust-analyzer
                clippy

                dprint
                typos
                zizmor

                # Used in sample script
                curl
                jq
                findutils # xargs
              ]
            );
          };
        }
      );
    };
}
