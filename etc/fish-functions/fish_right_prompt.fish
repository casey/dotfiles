function fish_right_prompt

	set_color brblue
  echo -n (prompt_pwd)
  set_color normal

  if __fish_is_git_repository
    if git_untracked_files
      set_color red
    else if git_worktree_dirty
      set_color red
    else if not git diff --cached --quiet > /dev/null 2> /dev/null
      set_color green
    else
	    set_color brblue
    end

    echo -n " ⟦"

    if git symbolic-ref -q HEAD > /dev/null 2> /dev/null
      git branch --show-current
    else
      echo DETACHED
    end

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
