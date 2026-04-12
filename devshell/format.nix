{ pkgs }:

pkgs.runCommandNoCC "check-format"
  {
    buildInputs = with pkgs; [
      fd

      shellcheck

      nixfmt
      prettier
      shfmt
      sleek
      taplo
      treefmt
    ];
  }
  ''
    treefmt \
      --allow-missing-formatter \
      --fail-on-change \
      --no-cache \
      --formatters prettier \
      --formatters nix \
      --formatters shell \
      --formatters toml \
      -C ${./..}

    echo

    # it worked!
    touch $out
  ''
