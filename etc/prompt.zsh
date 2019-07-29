# prompt
if [[ "$terminfo[colors]" -ge 8 ]]; then
  colors
fi

# nice color names
for color in RED GREEN YELLOW BLUE MAGENTA CYAN WHITE; do
  eval PR_$color='%{$terminfo[bold]$fg[${(L)color}]%}'
  eval PR_LIGHT_$color='%{$fg[${(L)color}]%}'
done

# display host if we're sshing
if [[ -n "$SSH_CONNECTION" ]]; then
  H="%m"
fi

# scary red prompt if root
if [[ $UID == 0 ]]; then
  PROMPT_COLOR=$PR_RED
else
  PROMPT_COLOR=$PR_LIGHT_BLUE
fi

# clear color
CLEAR_COLOR="%b%f"

# function to set PS1
ps1() {
  local MODE="${${KEYMAP/vicmd/·}/(main|viins)/:}"
  local BACKGROUND_MODE="${${KEYMAP/vicmd/∗}/(main|viins)/⁑}"
  PS1="%0(?.$PROMPT_COLOR.$PR_LIGHT_YELLOW)$H%1(j.$BACKGROUND_MODE.$MODE) $CLEAR_COLOR"
}

# set ps1
ps1

# alternate prompts
PS2="$PROMPT_COLOR> $CLEAR_COLOR"
PS3="$PROMPT_COLOR? $CLEAR_COLOR"
PS4="$PROMPT_COLOR+ $CLEAR_COLOR"

RPS1="$PROMPT_COLOR%~$CLEAR_COLOR"

# redraw prompt on line editor start and keymap select
function redraw-prompt {
  ps1
  # re-expand prompt
  zle reset-prompt
}

zle -N zle-line-init redraw-prompt     # redraw prompt on line editor start
zle -N zle-keymap-select redraw-prompt # redraw prompt when keymap changes
