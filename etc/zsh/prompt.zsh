colors

PROMPT_CLEAR=%b%f
PROMPT_RED="%{$fg[red]%}"
PROMPT_GREEN="%{$fg[green]%}"
PROMPT_BLUE="%{$fg[blue]%}"
PROMPT_YELLOW="%{$fg[yellow]%}"

PS1='$(prompt-color)$(prompt-left)$PROMPT_CLEAR'
PS2='$(prompt-color)> $PROMPT_CLEAR'
PS3='$(prompt-color)? $PROMPT_CLEAR'
PS4='$(prompt-color)+ $PROMPT_CLEAR'
RPS1='$(prompt-right)'
