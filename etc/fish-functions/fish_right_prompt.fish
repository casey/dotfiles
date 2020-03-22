function fish_right_prompt

	set_color brblue
  echo -n (prompt_pwd)
  set_color normal

  if __fish_is_git_repository
    if not git diff --quiet
      set_color red
    else if not git diff --cached --quiet
      set_color green
    else
	    set_color brblue
    end

    echo -n " ⟦"

    git branch --show-current

    if git rev-parse -q --verify refs/stash > /dev/null
      set stashes (git rev-list --walk-reflogs --count refs/stash)

      if test $stashes -gt 0
        echo -n "|$stashes"
      end
    end

    echo -n "⟧"

    set_color normal
  end
end
