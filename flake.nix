{
  description = "Axdot";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    (flake-utils.lib.eachDefaultSystem

      (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ self.overlay ];
          };
        in
        rec {
          packages.tunka = pkgs.callPackage ./default.nix { };
          defaultPackage = packages.tunka;
          apps.tunka = flake-utils.lib.mkApp {
            drv = packages.tunka;
            exePath = "/bin/tunka";
          };
          defaultApp = apps.tunka;
          devShell = pkgs.callPackage ./shell.nix { };

        })) // {
      overlay = final: prev: {
        tunka = final.callPackage ./default.nix { };
      };
    };
}
