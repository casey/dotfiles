function git_untracked_files
  git ls-files --exclude-standard --others --error-unmatch . > /dev/null 2> /dev/null
end
