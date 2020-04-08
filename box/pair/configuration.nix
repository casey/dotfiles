{ config, pkgs, ... }:
{
  imports = [
    ./hardware-configuration.nix
  ];

  nixpkgs.config = {
    allowUnfree = true;
  };

  boot = {
    cleanTmpDir = true;

    loader.grub = {
      enable     = true;
      devices    = ["/dev/vda"];
      version    = 2;
    };
  };

  time.timeZone = "America/Los_Angeles";

  networking = {
    hostName = "pair";
    firewall = {
      enable          = true;
      allowPing       = true;
      allowedTCPPorts = [];
      allowedUDPPorts = [];
    };
  };

  console = {
    font         = "Lat2-Terminus16";
    useXkbConfig = true;
  };

  i18n = {
    defaultLocale = "en_US.UTF-8";
  };

  security.sudo.wheelNeedsPassword = false;

  services = {
    mingetty.greetingLine = ''Welcome!'';

    xserver = {
      layout     = "us";
      xkbVariant = "altgr-intl";
      xkbOptions = "ctrl:nocaps, compose:rwin, numpad:mac, shift:both_capslock_cancel, lv3:ralt_switch";
    };

    sysstat = {
      enable = true;
    };

    openssh.enable = true;
  };

  users.extraUsers.pair = {
    extraGroups  = [
      "libvirtd"
      "uucp"
      "wheel"
    ];
    isNormalUser = true;
    shell        = pkgs.zsh;
    uid          = 1000;
    openssh.authorizedKeys.keys = [
      "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIPlE0aeKgtJO5hE2SOWPzCOk3ht1mKUPNXUyMK9B7zT4 durandal"
    ];
  };

  environment = {
    systemPackages = with pkgs; [
      bat
      bc
      binutils
      cargo-edit
      cargo-watch
      clang
      cmake
      colordiff
      dash
      ed
      exa
      fd
      file
      fzf
      gitAndTools.diff-so-fancy
      gitAndTools.gitFull
      gnumake
      gnupg
      gopher
      htop
      imagemagick
      just
      ncurses
      neofetch
      nmap
      nodejs
      parted
      perl
      pkgconfig
      python38
      qemu
      ranger
      ripgrep
      rustup
      sysstat
      tcpdump
      tcptrack
      telnet
      tig
      tmux
      traceroute
      unrar
      unzip
      urbit
      vim
      wget
      whois
      ympd
      zlib
      zsh
    ];
  };

  system.stateVersion = "18.09";
}
