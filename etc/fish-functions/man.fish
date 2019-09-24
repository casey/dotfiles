function man
  set -x LESS_TERMCAP_md (tput bold; tput setaf 4)
  set -x LESS_TERMCAP_me (tput sgr0)
  set -x LESS_TERMCAP_mb (tput blink)
  set -x LESS_TERMCAP_us (tput setaf 2)
  set -x LESS_TERMCAP_ue (tput sgr0)
  set -x LESS_TERMCAP_so (tput smso)
  set -x LESS_TERMCAP_se (tput rmso)
  set -x PAGER less
  env man $argv
end
