{ config, lib, pkgs, ... }: {
  imports = [
    ./hardware-configuration.nix
  ];

  boot = {
    initrd.luks.devices = {
      "luks-7674a91c-ec25-4e98-9898-fa0e300909e4" = {
        device = "/dev/disk/by-uuid/7674a91c-ec25-4e98-9898-fa0e300909e4";
      };
    };
    loader = {
      systemd-boot.enable = true;
      efi.canTouchEfiVariables = true;
    };
    tmp.cleanOnBoot = true;
  };

  environment = {
    systemPackages = with pkgs; [
      btop
      git
      just
      ripgrep
      speedtest-cli
      tmux
      vim
    ];
    variables.EDITOR = "vim";
  };

  i18n = {
    defaultLocale = "en_US.UTF-8";
    extraLocaleSettings = {
      LC_ADDRESS = "en_US.UTF-8";
      LC_IDENTIFICATION = "en_US.UTF-8";
      LC_MEASUREMENT = "en_US.UTF-8";
      LC_MONETARY = "en_US.UTF-8";
      LC_NAME = "en_US.UTF-8";
      LC_NUMERIC = "en_US.UTF-8";
      LC_PAPER = "en_US.UTF-8";
      LC_TELEPHONE = "en_US.UTF-8";
      LC_TIME = "en_US.UTF-8";
    };
  };

  networking = {
    hostName = "z";
    networkmanager.enable = true;
  };

  nixpkgs.config.allowUnfree = true;

  nix.settings.experimental-features = [
    "flakes"
    "nix-command"
  ];

  security = {
    sudo = {
      execWheelOnly = true;
      wheelNeedsPassword = false;
    };
  };

  services = {
    avahi = {
      enable = true;
      nssmdns4 = true;
      nssmdns6 = true;
      publish = {
        enable = true;
        addresses = true;
      };
    };

    openssh = {
      enable = true;
      settings = {
        PasswordAuthentication = false;
        PermitRootLogin = "no";
      };
    };

    xserver.xkb = {
      layout = "us";
      variant = "";
    };
  };

  system = {
    stateVersion = "24.11";
  };

  time.timeZone = "America/Los_Angeles";

  users = {
    users.rodarmor = {
      extraGroups = [ "networkmanager" "wheel" ];
      isNormalUser = true;
      openssh.authorizedKeys.keys = [
        "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIFbSqH7DNg3/USFtrLG183EVmL7VH7v+92qMbRvlOpSy rodarmor@odin"
      ];
      packages = with pkgs; [];
      uid = 1000;
    };
  };
}
