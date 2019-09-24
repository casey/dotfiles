function fish_prompt
  if test $status -ne 0
    set_color yellow
  else if test (id -u) -eq 0
    set_color red
  else
    set_color blue
  end

  if set -q SSH_CONNECTION
    echo -n (hostname)
  end

  if jobs -q
    switch $fish_bind_mode
      case insert
        echo -n "⁑ "
      case default
        echo -n "∗ "
      case visual
        echo -n "∗ "
    end
  else
    switch $fish_bind_mode
      case insert
        echo -n ": "
      case default
        echo -n "· "
      case visual
        echo -n "· "
    end
  end

  set_color normal
end
