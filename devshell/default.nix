{ rustToolchain
, cargoArgs
, unitTestArgs
, pkgs
, lib
, stdenv
, darwin
, ...
}:

let
  cargo-ext = pkgs.callPackage ./cargo-ext.nix { inherit cargoArgs unitTestArgs; };
in
pkgs.mkShell {
  name = "dev-shell";

  buildInputs = lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.Security
    darwin.apple_sdk.frameworks.SystemConfiguration
  ];

  nativeBuildInputs = with pkgs; [
    cargo-ext.cargo-build-all
    cargo-ext.cargo-clippy-all
    cargo-ext.cargo-doc-all
    cargo-ext.cargo-nextest-all
    cargo-ext.cargo-test-all
    cargo-ext.cargo-udeps-all
    cargo-ext.cargo-watch-all
    cargo-nextest
    cargo-udeps
    cargo-watch
    rustToolchain

    jq

    hclfmt
    nixpkgs-fmt
    nodePackages.prettier
    shfmt
    sleek
    taplo
    treefmt

    shellcheck
  ];

  shellHook = ''
    export NIX_PATH="nixpkgs=${pkgs.path}"
  '';
}
