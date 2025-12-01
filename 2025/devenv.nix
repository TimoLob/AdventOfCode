{ pkgs, lib, config, inputs, devenv-zsh, ... }:

{
  imports = [ devenv-zsh.plugin ];

  zsh.enable = true;
  zsh.extraInit = ''

  '';

  # https://devenv.sh/basics/
  env.GREET = "AoC";

  # https://devenv.sh/packages/
  packages = [ 
    pkgs.git 
    pkgs.openssl
  ];

  # https://devenv.sh/languages/
  languages.rust = {
    enable = true;
    channel = "stable";
    version = "latest";
  };

  # https://devenv.sh/processes/
  processes.cargo-watch.exec = "cargo-watch";

  # https://devenv.sh/services/
  # services.postgres.enable = true;

  enterShell = ''
  cargo install cargo-generate
  cargo --version
  '';

  # See full reference at https://devenv.sh/reference/options/
}
