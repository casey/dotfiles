bootstrap:
  ./bootstrap

init os name:
  ./os/{{os}}/init {{name}}

rustup:
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

watch +args='lcheck --all --all-targets':
  cargo watch --clear --exec '{{args}}'

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
  cargo install rust-analyzer

formulae:
  # packages
  brew install gh
  brew install python
  brew install reattach-to-user-namespace
  brew install tig
  brew install tmux
  brew install zsh
  # casks
  brew install iina
  brew install alacritty
  brew install google-chrome
  brew install quicksilver
  # fonts
  brew tap homebrew/cask-fonts
  brew install font-dejavu-sans-mono-nerd-font

services:
  brew services start bitcoin
  launchctl load ~/Library/LaunchAgents/com.rodarmor.bitcoin-regtest.plist
  launchctl load ~/Library/LaunchAgents/com.rodarmor.bitcoin-signet.plist
  launchctl load ~/Library/LaunchAgents/com.rodarmor.bitcoin-testnet.plist

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

disable-power-chime:
  defaults write com.apple.PowerChime ChimeOnNoHardware -bool true
  killall PowerChime

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
