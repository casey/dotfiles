function fish_prompt
  # red if root
  # yellow if command failed
  # username if sshing

  set_color brblue

  switch $fish_bind_mode
    case insert
      echo -n ": "
    case default
      echo -n "· "
    case visual
      echo -n "· "
  end

  set_color normal
end


# display host if we're sshing
# if [[ -n "$SSH_CONNECTION" ]]; then
#   H="%m"
# fi

# scary red prompt if root
# if [[ $UID == 0 ]]; then
#   PROMPT_COLOR=$PR_RED
# else
#   PROMPT_COLOR=$PR_LIGHT_BLUE
# fi

# local MODE="${${KEYMAP/vicmd/·}/(main|viins)/:}"
# local BACKGROUND_MODE="${${KEYMAP/vicmd/∗}/(main|viins)/⁑}"
# PS1="%0(?.$PROMPT_COLOR.$PR_LIGHT_YELLOW)$H%1(j.$BACKGROUND_MODE.$MODE) $CLEAR_COLOR"
