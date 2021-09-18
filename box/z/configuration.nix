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
  imports = [
    ./hardware-configuration.nix
    "${nix-bitcoin}/modules/modules.nix"
  ];

  nixpkgs.config.allowUnfree = true;

  system.stateVersion = "21.05";

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
          { devices = ["nodev"]; path = "/boot/esp0"; }
          { devices = ["nodev"]; path = "/boot/esp1"; }
        ];
      };
      systemd-boot.enable = false;
    };
  };

  networking = {
    defaultGateway = "51.255.87.254";
    firewall = {
      allowedTCPPorts = [80 443];
    };
    hostName = "z";
    interfaces.eno1.ipv4.addresses = [{address = "51.255.87.91"; prefixLength = 24;}];
    nameservers = [ "1.1.1.1" "1.0.0.0" "2606:4700:4700::1111" "2606:4700:4700::1001" ];
    useDHCP = false;
  };

  nix-bitcoin.generateSecrets = true;

  programs = {
    mosh.enable = true;
    zsh.enable = true;
  };

  security = {
    acme = {
      acceptTerms = true;
      email = "casey@rodarmor.com";
    };
    sudo = {
      enable = true;
      execWheelOnly = true;
      wheelNeedsPassword = false;
    };
  };

  users = {
    defaultUserShell = pkgs.zsh;

    users = {
      root = {
        initialHashedPassword = "";
        openssh.authorizedKeys.keys = ssh-keys;
      };

      rodarmor = {
        extraGroups = ["wheel"];
        isNormalUser = true;
        openssh.authorizedKeys.keys = ssh-keys;
        uid = 1000;
      };
    };
  };

  nix-bitcoin.operator = {
    enable = true;
    name = "rodarmor";
  };

  services = {
    bitcoind = {
      enable = true;
    };

    lnd = {
      enable = true;
    };

    openssh = {
      enable = true;
      permitRootLogin = "prohibit-password";
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
    python39Full
    ripgrep
    rustup
    tmux
    pinentry-curses
  ];
}
