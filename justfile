default:
	git submodule update --init --recursive .
	./submodules/dotbot/bin/dotbot -c default.yaml

rustup:
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

homebrew:
	/usr/bin/ruby -e "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/master/install)"

rust:
	rustup install nightly
	rustup component add rustfmt

crates:
	cargo install --force ripgrep
	cargo install --force exa
	cargo install --force bat
	cargo install --force fd-find
	cargo install --force remote
	cargo install --force just
	cargo install --force cargo-watch
	cargo install --force cargo-check
	cargo install --force cargo-outdated
	cargo install --force cargo-edit
	cargo +nightly install --force racer

formulae:
	brew install cmake
	brew install python
	brew install reattach-to-user-namespace
	brew install tmux
	brew install zsh
	brew install gpg
	brew install macvim
	brew cask install vlc
	brew cask install alacritty
	brew cask install google-chrome
	brew cask install quicksilver
	brew tap homebrew/cask-fonts
	brew cask install font-dejavu-sans-mono-for-powerline
	# yabai and skhd
	brew tap koekeishiya/formulae
	brew install --HEAD yabai
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
