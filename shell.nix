with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "tunka-dev";

  RUST_BACKTRACE = 1;

  nativeBuildInputs = [ rustup just ];
}
