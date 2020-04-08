{ config, pkgs, ... }:
with pkgs.stdenv.lib;
{
  imports = [
    ./hardware-configuration.nix
  ];

  nix = {
    useSandbox = true;
    nixPath = [
      "/etc/nixos"
      "nixos-config=/etc/nixos/configuration.nix"
    ];
  };

  nixpkgs.config = {
    allowUnfree = true;

    packageOverrides = super: let self = super.pkgs; in {
      polybar = super.polybar.override {
        mpdSupport = true;
      };
    };
  };

  boot = {
    cleanTmpDir = true;
    loader = {
      systemd-boot.enable      = true;
      efi.canTouchEfiVariables = true;
      grub = {
        enable           = true;
        device           = "nodev";
        efiSupport       = true;
        gfxmodeBios      = "2560x1440";
        gfxmodeEfi       = "2560x1440";
        version          = 2;
        memtest86 = {
          enable = true;
          params = ["btrace"];
        };
      };
    };

    kernelParams = [ "consoleblank=0" ];

    initrd.luks.devices = [
      {
        allowDiscards = true;
        device        = "/dev/disk/by-uuid/0b815c98-a70f-4cca-8497-f46685d144e2";
        name          = "root";
        preLVM        = true;
      }
      {
        allowDiscards = true;
        device        = "/dev/disk/by-uuid/9b8af2f8-da3c-47be-bd11-c1730fcd2df1";
        name          = "home";
        preLVM        = true;
      }
    ];
  };

  fileSystems = [
    {
      device     = "/dev/disk/by-label/nixos-home";
      mountPoint = "/home";
    }
  ];

  time.timeZone = "America/Los_Angeles";

  networking = {
    hostName = "fab";
    networkmanager.enable = true;
    firewall = {
      enable          = true;
      allowPing       = false;
      allowedTCPPorts = [
        51413
        # config.services.mpd.network.port
      ];
      allowedUDPPorts = [
        51413
      ];
    };
    nameservers = [
      "1.1.1.1"
      "1.0.0.0"
      # "2606:4700:4700::1111"
      # "2606:4700:4700::1001"
    ];
    extraHosts = ''
    0.0.0.0 clientupdates.dropboxstatic.com
    '';
  };

  i18n = {
    consoleFont         = "Lat2-Terminus16";
    defaultLocale       = "en_US.UTF-8";
    consoleUseXkbConfig = true;
  };

  hardware = {
    pulseaudio = {
      enable = true;
      tcp = {
        enable = true;
        anonymousClients.allowedIpRanges = [ "127.0.0.1" ];
      };
      extraConfig = ''
        update-sink-proplist alsa_output.pci-0000_12_00.3.analog-stereo device.description="Headphones"
        update-sink-proplist alsa_output.usb-Burr-Brown_from_TI_USB_Audio_DAC-00.analog-stereo device.description="Speakers"
      '';
    };

    opengl.driSupport32Bit = true;
  };

  fonts = {
    enableFontDir = true;
    enableGhostscriptFonts = true;
    fonts = with pkgs; [
      andagii
      baekmuk-ttf
      corefonts
      dejavu_fonts
      font-awesome-ttf
      inconsolata
      nafees
      nerdfonts
      noto-fonts-cjk
      powerline-fonts
      profont
      siji
      terminus_font
      tewi-font
      unifont
      unifont_upper
    ];
  };

  security.sudo.wheelNeedsPassword = false;

  systemd.mounts = [
    {
      requiredBy = ["transmission.service"];
      before     = ["transmission.service"];
      type       = "none";
      what       = "/home/rodarmor/dat/transmission";
      where      = "/var/bind/transmission";
      options    = concatStringsSep "," [
        "bind"
        "uid=${toString config.ids.uids.transmission}"
        "gid=${toString config.ids.gids.transmission}"
      ];
    }
    {
      requiredBy = ["mpd.service"];
      before     = ["mpd.service"];
      type       = "none";
      what       = "/home/rodarmor/Dropbox/music/playlists";
      where      = "/var/bind/mpd/playlists";
      options    = concatStringsSep "," [
        "bind"
        "uid=${toString config.ids.uids.mpd}"
        "gid=${toString config.ids.gids.mpd}"
      ];
    }
    {
      requiredBy = ["mpd.service"];
      before     = ["mpd.service"];
      type       = "none";
      what       = "/home/rodarmor/Dropbox/music/flac";
      where      = "/var/bind/mpd/music";
      options    = "bind,ro";
    }
  ];

  services = {
    mingetty.greetingLine = ''Welcome!'';

    pcscd = {
      enable = true;
    };

    compton.enable = true;

    xserver = {
      enable = true;
      videoDrivers = [ "nvidia" ];

      desktopManager = {
        default = "xfce";
        xfce = {
          enable = true;
          thunarPlugins = [ pkgs.xfce.thunar-archive-plugin ];
          screenLock = "slock";
        };
      };

      displayManager = {
        sessionCommands = ''
          ~/bin/xmerge
          xbindkeys
          dropbox &
          xset s off -dpms
          xkbcomp ~/src/local/etc/layout.xkb $DISPLAY
        '';

        sddm = {
          enable            = true;
          autoNumlock       = true;
          autoLogin.enable  = true;
          autoLogin.relogin = true;
          autoLogin.user    = "rodarmor";
        };
      };

      xautolock = {
        enable = true;
        locker = "/run/wrappers/bin/slock";
        time   = 15;
      };

      layout     = "us";
      xkbVariant = "altgr-intl";
      xkbOptions = "ctrl:nocaps, compose:rwin, numpad:mac, shift:both_capslock_cancel, lv3:ralt_switch";

      screenSection                         = ''
      Option "metamodes" "2560x1440_144 +0+0"
      '';
    };

    locate = {
      enable    = true;
      locate    = pkgs.mlocate;
      interval  = "hourly";
      localuser = null;
    };

    printing = {
      enable        = true;
      browsing      = true;
      defaultShared = true;
    };

    gnome3.at-spi2-core = {
      enable = true;
    };

    /*
    openvpn = {
      servers = {
        pia = {
          autoStart = false;
          config    = readFile "/etc/nixos/pia/US Silicon Valley.ovpn";
        };
      };
    };
    */

    snapper = {
      snapshotInterval = "hourly";
      configs.home = {
        subvolume = "/home";
        extraConfig = ''
          # create hourly snapshots
          TIMELINE_CREATE="yes"

          # cleanup hourly snapshots after some time
          TIMELINE_CLEANUP="yes"

          # limits for timeline cleanup
          TIMELINE_MIN_AGE="1800"
          TIMELINE_LIMIT_HOURLY="10"
          TIMELINE_LIMIT_DAILY="10"
          TIMELINE_LIMIT_WEEKLY="0"
          TIMELINE_LIMIT_MONTHLY="10"
          TIMELINE_LIMIT_YEARLY="10"
        '';
      };
    };

    udev.extraRules = ''
      SUBSYSTEMS=="usb", ATTRS{idVendor}=="2581", ATTRS{idProduct}=="1b7c", MODE="0660", GROUP="wheel"
      SUBSYSTEMS=="usb", ATTRS{idVendor}=="2581", ATTRS{idProduct}=="2b7c", MODE="0660", GROUP="wheel"
      SUBSYSTEMS=="usb", ATTRS{idVendor}=="2581", ATTRS{idProduct}=="3b7c", MODE="0660", GROUP="wheel"
      SUBSYSTEMS=="usb", ATTRS{idVendor}=="2581", ATTRS{idProduct}=="4b7c", MODE="0660", GROUP="wheel"
      SUBSYSTEMS=="usb", ATTRS{idVendor}=="2581", ATTRS{idProduct}=="1807", MODE="0660", GROUP="wheel"
      SUBSYSTEMS=="usb", ATTRS{idVendor}=="2581", ATTRS{idProduct}=="1808", MODE="0660", GROUP="wheel"
      SUBSYSTEMS=="usb", ATTRS{idVendor}=="2c97", ATTRS{idProduct}=="0000", MODE="0660", GROUP="wheel"
      SUBSYSTEMS=="usb", ATTRS{idVendor}=="2c97", ATTRS{idProduct}=="0001", MODE="0660", GROUP="wheel"
    '';

    sysstat = {
      enable = true;
    };

    transmission = {
      enable   = true;
      settings = {
        download-dir           = "/var/bind/transmission/downloads";
        incomplete-dir         = "/var/bind/transmission/incomplete";
        incomplete-dir-enabled = true;
        rpc-whitelist          = "127.0.0.1";
        umask                  = 002;

        # bind-address-ipv4: String (default = "0.0.0.0") Where to listen for peer connections.
        # bind-address-ipv6: String (default = "::") Where to listen for peer connections.

      };
    };

    mpd = {
      enable            = true;
      musicDirectory    = "/var/bind/mpd/music";
      playlistDirectory = "/var/bind/mpd/playlists";

      network = {
        port          = 6600;
        listenAddress = "0.0.0.0";
      };

      extraConfig    = ''
        auto_update                      "yes"
        mixer_type                       "software"
        restore_paused                   "yes"
        save_absolute_paths_in_playlists "no"

        audio_output {
          name    "Headphones"
          type    "alsa"
          device  "hw:1,0"
        }

        audio_output {
          name    "Speakers"
          type    "alsa"
          device  "hw:2,0"
        }

        audio_output {
          name   "PulseAudio"
          type   "pulse"
          server "127.0.0.1"
        }

        audio_output {
          type   "fifo"
          name   "Visualizer"
          path   "${config.services.mpd.dataDir}/fifo"
          format "44100:16:2"
        }

        audio_output {
          type      "httpd"
          name      "HTTP Stream"
          encoder   "vorbis"
          port		  "6601"
          bitrate   "128"
          format    "44100:16:1"
          always_on "yes"
          tags      "yes"
        }
      '';
    };

    udev.packages = [ pkgs.android-udev-rules ];

    xbanish = {
      enable = true;
    };
  };

  virtualisation = {
    libvirtd.enable    = true;
  };

  system.nixos.stateVersion = "17.03";

  users.extraUsers.rodarmor = {
    extraGroups  = [
      "libvirtd"
      "mlocate"
      "networkmanager"
      "transmission"
      "mpd"
      "uucp"
      "wheel"
      "audio"
    ];
    isNormalUser = true;
    shell        = "/run/current-system/sw/bin/zsh";
    uid          = 1000;
  };

  programs = {
    slock.enable = true;
  };

  environment = {
    systemPackages = with pkgs; [
      adapta-gtk-theme
      alacritty
      arc-icon-theme
      aria
      artha
      atom
      avahi
      baobab
      bc
      binutils
      breeze-icons
      cantata
      cmake
      colordiff
      compton
      cups
      dash
      dmenu
      dockbarx
      dropbox
      ecm
      ed
      entr
      exa
      feh
      ffmpeg
      file
      filelight
      firefox
      fontconfig
      fontmatrix
      freetype
      fswatch
      fzf
      gcc
      gcolor2
      gimp
      gitAndTools.gitFull
      gnome3.dconf
      gnome3.file-roller
      gnome3.gedit
      gnome3.gnome-boxes
      gnome3.gnome-system-monitor
      gnome3.gucharmap
      gnome3.networkmanagerapplet
      gnumake
      gnupg
      google-chrome
      gopher
      gpa
      gsimplecal
      gtk-engine-murrine
      gtk_engines
      haskellPackages.xmobar
      htop
      hwinfo
      hyper
      i3
      i3lock
      icu
      imagemagick
      inotifyTools
      kwin
      libiconv
      libmtp
      lsof
      materia-theme
      moka-icon-theme
      mpc_cli
      ncmpcpp
      ncurses
      neofetch
      nix
      nixUnstable
      nmap
      nodejs-8_x
      numix-gtk-theme
      numlockx
      okular
      openbox
      paper-icon-theme
      papirus-icon-theme
      paprefs
      parted
      pavucontrol
      picard
      pkgconfig
      playerctl
      pnmixer
      polybar
      qemu
      ranger
      rc
      rclone
      readline70
      rofi
      rxvt_unicode
      screenfetch
      scrot
      sddm
      siji
      spotify
      stalonetray
      steam
      sudo
      sysstat
      taffybar
      tcpdump
      tcptrack
      telnet
      tint2
      tmux
      traceroute
      transmission-remote-cli
      transmission_remote_gtk
      trayer
      tup
      unrar
      unzip
      urbit
      vimHugeX
      vimpc
      virtmanager
      vlc
      wget
      whois
      windowlab
      wmctrl
      wordnet
      xautolock
      xbindkeys
      xclip
      xdg-user-dirs
      xdotool
      xfce.xfce4_dockbarx_plugin
      xfce.xfce4panel
      xfce.xfce4session
      xfce.xfce4settings
      xfce.xfce4taskmanager
      xfce.xfconf
      xfce.xfwm4
      xfce.xfwm4themes
      xfe
      xfontsel
      xlsfonts
      xorg.libxcb
      xorg.xev
      xorg.xkbcomp
      xorg.xkill
      ympd
      zlib
      zsh
      (python27.withPackages(ps: with ps; [
        numpy ipython pip pyyaml pyperclip sh psutil
        jinja2 dateutil lxml gnureadline mutagen pillow
      ]))
      (python36.withPackages(ps: with ps; [
        numpy ipython pip pyyaml pyperclip sh psutil
        jinja2 dateutil lxml gnureadline mutagen pillow
      ]))
    ];
  };
}
