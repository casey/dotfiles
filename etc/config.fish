# path variable
path: ~/bin                           # artisanal binaries
path: ~/sbin                          # admin binaries
path: ~/.cargo/bin                    # cargo
path: ~/.fzf/bin                      # fzf binary
path: ~/.nix-profile/bin              # nix env binaries
path: ~/.vim/plugged/vim-superman/bin # vman script
path: ~/opt/npm/bin                   # npm binaries
path: /usr/local/bin                  # homebrew binaries
path: /usr/local/sbin                 # admin homebrew binaries
path: /usr/local/opt/bin              # keg-only homebrew binaries
path: /usr/bin                        # binaries
path: /usr/sbin                       # admin binaries
path: /usr/games                      # games
path: /bin                            # core binaries
path: /sbin                           # core admin binaries

# environment variables
set -x CLICOLOR   1              # use color
set -x EDITOR     vim            # vim as editor
set -x HOST       (hostname)     # set HOST variable
set -x LANG       en_US.UTF-8    # use the english utf-8 locale
set -x LESS       -ifRMx2        # default options for less
set -x LS_OPTIONS '--color=auto' # make ls use color
set -x PAGER      less           # less as pager
set -x VISUAL     vim            # vim as visual editor

# use ripgrep with fzf
set -x FZF_DEFAULT_COMMAND "rg --files --hidden --glob '!.git/*' --glob '!*.pyc'"

# tell ripgrep to use a config file
set -x RIPGREP_CONFIG_PATH "$HOME/.ripgreprc"

# rust src path for use with rusty-tags
set -x RUST_SRC_PATH (rustc --print sysroot)/lib/rustlib/src/rust/src/

# aliases
alias ls   exa
alias diff colordiff
alias grep 'grep --color=auto'
alias rwd  'cd (pwd -P)'

# tty configuration
stty -ixoff      # disable sending start/stop characters
stty -ixon       # disable start/stop output control
stty start undef # unmap c-q
stty stop undef  # unmap c-s

# source dircolors
if test -f ~/.dir_colors/dircolors
  eval (dircolors ~/.dir_colors/dircolors)
end

# source base16 configuration
if status --is-interactive
  set BASE16_SHELL "$HOME/src/local/submodules/base16-shell/"
  source "$BASE16_SHELL/profile_helper.fish"
end
