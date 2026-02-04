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
  };

  outputs =
    {
      nixpkgs,
      ...
    }:
    let
      lib = nixpkgs.lib;
      forAllSystems = lib.genAttrs lib.systems.flakeExposed;
    in
    {
      formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.nixfmt-tree);

      packages = forAllSystems (
        system:
        let
          pkgs = nixpkgs.legacyPackages.${system};
        in
        rec {
          nixpkgs-update-log-checker = pkgs.callPackage ./package.nix { };
          default = nixpkgs-update-log-checker;
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
