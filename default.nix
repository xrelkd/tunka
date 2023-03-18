{ lib
, rustPlatform
, installShellFiles
}:

rustPlatform.buildRustPackage rec {
  pname = "tunka";
  version = "0.3.0";

  src = lib.cleanSource ./.;

  cargoLock.lockFile = ./Cargo.lock;

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
