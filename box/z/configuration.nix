# hints:
# - man configuration.nix
# - nixos-help

{ config, pkgs, ... }:

{
  imports = [ ./hardware-configuration.nix ];

  boot = {
    loader = {
      systemd-boot.enable = true;
      efi.canTouchEfiVariables = true;
    };
    initrd.luks.devices."luks-7674a91c-ec25-4e98-9898-fa0e300909e4".device = "/dev/disk/by-uuid/7674a91c-ec25-4e98-9898-fa0e300909e4";
  };

  networking = {
    hostName = "z";
    networkmanager.enable = true;
  };

  time.timeZone = "America/Los_Angeles";

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

    bitcoind = {
      mainnet = {
        enable = true;
	extraConfig = ''
          blockfilterindex=1
          coinstatsindex=1
          txindex=1
        '';
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

  users.users.rodarmor = {
    extraGroups = [ "networkmanager" "wheel" ];
    isNormalUser = true;
    openssh.authorizedKeys.keys = [
      "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIFbSqH7DNg3/USFtrLG183EVmL7VH7v+92qMbRvlOpSy rodarmor@odin"
    ];
    packages = with pkgs; [];
    uid = 1000;
  };

  nixpkgs.config.allowUnfree = true;

  environment = {
    systemPackages = with pkgs; [
      git
      speedtest-cli
      tmux
      vim
    ];
    variables.EDITOR = "vim";
  };

  system.stateVersion = "24.11";
}
