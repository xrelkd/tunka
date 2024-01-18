{ name
, version
, dockerTools
, tunka
, buildEnv
, ...
}:

dockerTools.buildImage {
  inherit name;
  tag = "v${version}";

  copyToRoot = buildEnv {
    name = "image-root";
    paths = [ tunka ];
    pathsToLink = [ "/bin" ];
  };

  config = {
    Entrypoint = [ "${tunka}/bin/tunka" ];
  };
}
