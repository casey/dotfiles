function path:
  if not test -d $argv[1];
    return
  end

  if contains $argv[1] $PATH
    return
  end
  
  set PATH $argv[1] $PATH
end
