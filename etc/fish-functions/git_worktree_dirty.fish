function git_worktree_dirty
  not git diff --quiet > /dev/null 2> /dev/null
end
