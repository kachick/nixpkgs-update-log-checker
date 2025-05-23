{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
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
                nixfmt-rfc-style
                nixd

                rustc
                cargo
                rustfmt
                rust-analyzer
                clippy

                dprint
                typos

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
