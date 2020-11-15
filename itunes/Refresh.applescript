property my_title : "Refresh"

tell application "iTunes"
  repeat with aTrack in selection
    try
      refresh aTrack
    end try
  end repeat
end tell
