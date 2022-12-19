# Ⅰ
alias: f 'fd'
alias: g 'git'
alias: j 'just'
alias: m 'vman'
alias: v 'vim-fzf'

# Ⅱ
alias: .. 'cd ..'
alias: .j 'j --justfile ~/.justfile --working-directory ~'
alias: a1 'alias-list 1'
alias: a2 'alias-list 2'
alias: al 'alias-list 2'
alias: ea 'vim ~/.zsh/aliases.zsh'
alias: ee 'vim ~/.vimrc'
alias: ej 'vim ~/.user.justfile'
alias: em 'vim ~/.mutt/muttrc'
alias: es 'vim ~/.zshrc'
alias: et 'vim ~/.tmux.conf'
alias: ev 'vim ~/.vimrc'
alias: kb 'keybase'
alias: kc 'keybase-chat'
alias: la 'exa --across --all'
alias: lg 'lazygit'
alias: ll 'exa --long --binary --group --git'
alias: ls 'exa'
alias: p1 'ping 1.1.1.1'
alias: p6 'ping6 2606:4700:4700::1111'
alias: p8 'ping 8.8.8.8'
alias: pw 'curl https://rodarmor.com'
alias: sl 'ls'
alias: tb 'nc termbin.com 9999'
alias: tp 'term --command ssh pair@pair.rodarmor.com'
alias: zf 'zsh-edit-function'

# Ⅲ
alias: cwt 'cargo watch --clear --shell "cargo ltest --color always --bin ord -- --color always 2>&1 | less -r"'
alias: how 'howdoi -a'
alias: psg 'ps aux | grep'
alias: rwd 'cd `pwd -P`'

# Ⅳ
alias: Kill 'kill -9'
alias: cd.. 'cd ..'
alias: diff 'colordiff'
alias: grep 'grep --color=auto'
alias: help 'run-help'
alias: mutt 'neomutt'
alias: pare 'tr -s "[:blank:]" " " | cut -d " " -f'
alias: port 'port -vuRc'
alias: post 'cd ~/src/blog && just post'
alias: sane 'stty sane'

# Ⅴ
alias: egrep 'egrep --color=auto'
alias: eject 'drutil tray eject'

# Ⅸ
alias: colortest '~/src/local/submodules/base16-shell/colortest'
alias: post-file 'cd ~/src/blog && just file'

# git
alias: gc 'git commit'

alias: gaa 'git commit --amend --all'
alias: gac 'git commit --amend'
alias: gad 'git add'
alias: gap 'git add -p'
alias: gbD 'git branch --delete --force'
alias: gba 'git branch --all'
alias: gbd 'git branch --delete'
alias: gbm 'git branch -m'
alias: gbr 'git branch --verbose'
alias: gca 'git commit --all'
alias: gcb 'git checkout -b'
alias: gcf 'git config --local --unset rodarmor.slow true'
alias: gcm 'git commit-with-message'
alias: gco 'git checkout'
alias: gcp 'git commit --patch'
alias: gcs 'git config --local --add rodarmor.slow true'
alias: gct 'git cat-file'
alias: gdc 'git diff --cached'
alias: gdf 'git diff'
alias: gdi 'git diff --cached'
alias: gfa 'git fetch --all --prune'
alias: gif 'git info'
alias: glg 'git lg'
alias: gls 'git log --all --pretty=tformat:%H -n 1 --grep'
alias: gpc 'gh pr create --web'
alias: gpf 'git push -f'
alias: gpl 'git pull -p'
alias: gpp 'git pull -p && git push'
alias: gps 'git push'
alias: grb 'git rebase'
alias: gri 'git rebase -i'
alias: grl 'git rebase'
alias: grm 'git rm'
alias: grs 'git restore'
alias: grt 'git remote'
alias: gss 'git status --short'
alias: gst 'git info'
alias: gsu 'git submodule update --init --recursive'
alias: gsw 'git switch'
alias: gtw 'git commit -m tweak'

alias: ghprco 'gh pr checkout'
alias: ghprcr 'gh pr create --web'
alias: ghprls 'gh pr list'

alias: gaptwps 'gap && gtw && gps'
