property my_title : "Refresh"

tell application "Music"
  repeat with aTrack in selection
    try
      refresh aTrack
    end try
  end repeat
end tell
