bootstrap:
  ./bootstrap

rustup:
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

homebrew:
  /usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"

crates:
  cargo install --force --git https://github.com/alopatindev/cargo-limit
  cargo install --path pkg/shell
  cargo install bat
  cargo install cargo-check
  cargo install cargo-edit
  cargo install cargo-outdated
  cargo install cargo-watch
  cargo install exa
  cargo install fd-find
  cargo install just
  cargo install nvim-send
  cargo install remote
  cargo install ripgrep
  cargo install src

formulae:
  # packages
  brew install cmake
  brew install fish
  brew install gpg
  brew install macvim
  brew install pinentry-mac
  brew install python
  brew install reattach-to-user-namespace
  brew install tig
  brew install tmux
  brew install zsh
  # casks
  brew cask install vlc
  brew cask install alacritty
  brew cask install google-chrome
  brew cask install quicksilver
  # fonts
  brew tap homebrew/cask-fonts
  brew install font-dejavu-sans-mono-for-powerline
  # yabai and skhd
  brew tap koekeishiya/formulae
  brew install yabai
  brew install skhd

services:
  brew services start koekeishiya/formulae/yabai
  brew services start koekeishiya/formulae/skhd

hunter:
  brew install gstreamer
  brew install gst-plugins-base
  brew install libffi
  brew install libmagic
  git clone https://github.com/rabite0/hunter.git ~/tmp/hunter
  PKG_CONFIG_PATH="/usr/local/opt/libffi/lib/pkgconfig" cargo +nightly install --path ~/tmp/hunter

ledger:
  pip3 install ledger_agent

gh-md-toc:
  curl https://raw.githubusercontent.com/\
    ekalinin/github-markdown-toc/master/gh-md-toc \
    > bin/gh-md-toc
  chmod +x bin/gh-md-toc

dia-x2160:
  #!/usr/bin/env bash
  set -euo pipefail
  DIR=rsc/dia/x2160
  rm -rf $DIR
  mkdir $DIR
  mogrify \
    -filter point -resize '1600%' \
    -background transparent -gravity center -extent 2160x2160 \
    -path $DIR rsc/dia/original/*.png
