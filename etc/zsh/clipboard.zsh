set-system-clipboard() { cpy }
get-system-clipboard() { pst }

if [[ `uname` != Darwin && ! -z $DISPLAY ]]; then
  set-system-selection() { sel }
  get-system-selection() { ynk }
else
  set-system-selection() {
    CUTBUFFER=`cat`
  }
  get-system-selection() {
    echo $CUTBUFFER
  }
fi

zle -N kill-line-to-system-clipboard
kill-line-to-system-clipboard() {
  zle kill-line
  print -rn -- "$CUTBUFFER" | set-system-clipboard
}

zle -N yank-from-system-clipboard
yank-from-system-clipboard() {
  CUTBUFFER=`get-system-clipboard`
  zle yank
}

zle -N kill-line-to-x-selection
kill-line-to-x-selection() {
  zle kill-line
  print -rn -- "$CUTBUFFER" | set-system-selection
}

zle -N yank-from-x-selection
yank-from-x-selection() {
  CUTBUFFER=`get-system-selection`
  zle yank
}
