let
  nix-bitcoin = builtins.fetchTarball {
    url = "https://github.com/fort-nix/nix-bitcoin/archive/v0.0.52.tar.gz";
    sha256 = "sha256-eRA5HinTNtNBo1VGQmNmI8EwmjDCiZBUj5d36j39lNM=";
  };
  ssh-keys = [
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAINnYzKwzFaNhQ6AtonmWMkKHXHxASdUJd815wyAOt6qw x"
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIOnH2m99ZQvTjE0mNUoRmwSfQ5BUMaGrlVnwN/8NN5NW chromatic-abberation"
    "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIPlE0aeKgtJO5hE2SOWPzCOk3ht1mKUPNXUyMK9B7zT4 durandal"
  ];
in
{config, pkgs, ...}: {
  boot = {
    cleanTmpDir = true;
    loader = {
      efi = {
        canTouchEfiVariables = true;
        efiSysMountPoint = "/boot/efi";
      };
      grub = {
        enable = true;
        efiSupport = true;
        mirroredBoots = [
          {devices = ["nodev"]; path = "/boot/esp0";}
          {devices = ["nodev"]; path = "/boot/esp1";}
        ];
      };
      systemd-boot.enable = false;
    };
  };

  environment.systemPackages = with pkgs; [
    bat
    delta
    exa
    fd
    fzf
    gcc
    git
    gnupg
    just
    neovim
    pinentry-curses
    python39Full
    ripgrep
    rustup
    tmux
  ];

  imports = [
    ./hardware-configuration.nix
    "${nix-bitcoin}/modules/modules.nix"
  ];

  networking = {
    defaultGateway = "51.255.87.254";
    hostName = "z";
    interfaces.eno1.ipv4.addresses = [{address = "51.255.87.91"; prefixLength = 24;}];
    nameservers = ["1.1.1.1" "1.0.0.0" "2606:4700:4700::1111" "2606:4700:4700::1001"];
    useDHCP = false;
  };

  nix-bitcoin = {
    generateSecrets = true;
    operator = {
      enable = true;
      name = "rodarmor";
    };
  };

  nixpkgs.config.allowUnfree = true;

  programs = {
    mosh.enable = true;
    zsh.enable = true;
  };

  security = {
    sudo = {
      enable = true;
      execWheelOnly = true;
      wheelNeedsPassword = false;
    };
  };

  services = {
    bitcoind = {
      enable = true;
    };

    lnd = {
      enable = true;
      restOnionService = {
        enable = true;
      };
    };

    openssh = {
      enable = true;
      passwordAuthentication = false;
      permitRootLogin = "prohibit-password";
    };
  };

  system.stateVersion = "21.05";

  users = {
    defaultUserShell = pkgs.zsh;

    users = {
      rodarmor = {
        extraGroups = ["wheel"];
        isNormalUser = true;
        openssh.authorizedKeys.keys = ssh-keys;
        uid = 1000;
      };

      root = {
        initialHashedPassword = "";
        openssh.authorizedKeys.keys = ssh-keys;
      };
    };
  };
}
