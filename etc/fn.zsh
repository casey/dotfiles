path:() {
  local PARTS=(${(s/:/)PATH})
  if [[ -d "$1" ]]; then
    if [[ ${PARTS[(i)$1]} -le ${#PARTS} ]]; then
      # already in parts
    else
      PARTS+=("$1")
    fi
  fi
  export PATH=${(j/:/)PARTS}
}

bind:() {
  bindkey -M viins "$1" "$2"
  bindkey -M vicmd "$1" "$2"
}

arginfo() {
  echo $# arguments:

  for i in {1..$#} ; do
      echo $i: $argv[$i]
  done
}

f() {
  fd "$@"
}

can() {
  command -v $1 > /dev/null
  return $?
}

is-file() {
  [[ -f $1 ]]
}

up() {
  target=${1:-.git}
  d="`pwd`"

  while true; do
      if [[ -e "$d/$target" ]]; then
        cd "$d"
        return 0
      fi

      if [[ "$d" == / ]]; then
          echo "No $target found."
          return 1
      fi

      d=`dirname "$d"`
  done
}

exsh() {
  exec $SHELL
}

extract() {
    if [[ -z "$1" ]] ; then
        print -P "usage: \e[1;36mextract\e[1;0m < filename >"
        print -P "       Extract the file specified based on the extension"
    elif [[ -f $1 ]] ; then
        case ${(L)1} in
            *.tar.bz2)  tar -jxvf $1  ;;
            *.tar.gz)   tar -zxvf $1  ;;
            *.bz2)      bunzip2 $1    ;;
            *.gz)       gunzip $1     ;;
            *.jar)      unzip $1      ;;
            *.rar)      unrar x $1    ;;
            *.tar)      tar -xvf $1   ;;
            *.tbz2)     tar -jxvf $1  ;;
            *.tgz)      tar -zxvf $1  ;;
            *.zip)      unzip $1      ;;
            *.Z)        uncompress $1 ;;
            *)          echo "Unable to extract '$1' :: Unknown extension"
        esac
    else
        echo "File ('$1') does not exist!"
    fi
}

col-man() {
  man $@ | col -b | col -f | less
}

source:() {
  [[ -f $1 ]] && source $1
}

precmd() {
  pwd >> ~/.dir_history
  print -Pn "\e]0;zsh\a"
}

rd() {
  if [[ $# -lt 2 ]]; then
    echo "usage: rd OLD NEW [FLAGS]"
    return 1
  fi

  old=`pwd`
  new=`pwd | sed "s:$1:$2:$3"`
  echo $old '->' $new
  cd $new
}

forget-host() {
  if [[ $# -lt 1 ]]; then
    echo "usage: forget-host LINE_NUMBER"
    return 1
  fi

  sed -i '' -e "${1}d" ~/.ssh/known_hosts
}

get() {
  for item in "$@"; do
    aria2c -x 8 --file-allocation=none "$item"
  done
}

etc() {
  local file=`(cd ~/src/local && fzf)`
  if [[ -z $file ]]; then
    return 0
  fi
  vim ~/src/local/$file
}

tmp() {
  mkdir ~/tmp/$1
  pushd ~/tmp/$1
}

flag() {
 man $1 | less -rp "^ +$2"
}

path() {
  tr ':' '\n' <<< "$PATH"
}

if [[ -x /usr/libexec/pk-command-not-found ]]; then
  command_not_found_handler() {
    exec /usr/libexec/pk-command-not-found "$@"
  }
fi

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
  print -rn "$CUTBUFFER" | set-system-clipboard
}

zle -N yank-from-system-clipboard
yank-from-system-clipboard() {
  CUTBUFFER=`get-system-clipboard`
  zle yank
}

zle -N kill-line-to-x-selection
kill-line-to-x-selection() {
  zle kill-line
  print -rn "$CUTBUFFER" | set-system-selection
}

zle -N yank-from-x-selection
yank-from-x-selection() {
  CUTBUFFER=`get-system-selection`
  zle yank
}
