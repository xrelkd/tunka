{ name
, version
, lib
, stdenv
, rustPlatform
, installShellFiles
, darwin
}:

rustPlatform.buildRustPackage {
  pname = name;
  inherit version;

  doCheck = false;

  src = lib.cleanSource ./..;

  cargoLock = {
    lockFile = ../Cargo.lock;
  };

  buildInputs = lib.optionals stdenv.isDarwin [
    darwin.apple_sdk.frameworks.Security
    darwin.apple_sdk.frameworks.SystemConfiguration
  ];

  nativeBuildInputs = [ installShellFiles ];

  postInstall = ''
    installShellCompletion --cmd tunka \
      --bash <($out/bin/tunka completions bash) \
      --fish <($out/bin/tunka completions fish) \
      --zsh  <($out/bin/tunka completions zsh)
  '';

  meta = with lib; {
    homepage = "https://github.com/xrelkd/tunka";
    license = with licenses; [ gpl3 ];
    maintainers = with maintainers; [ xrelkd ];
  };
}
