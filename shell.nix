with import <nixpkgs> { };

stdenv.mkDerivation {
  name = "tunka-dev";
  nativeBuildInputs = [ rustup just ];
}
