function clipboard-cut
  set -l line (commandline)
  set -l cursor (math (commandline -C) + 1)
  set -l text (string sub -s $cursor $line)
  echo $text | cpy
  commandline -f kill-line
end
