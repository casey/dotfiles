property my_title : "Timestamp"

on timestamp(datetime)
	set command to "date -j -f '%A, %B %e, %Y at %H:%M:%S' '" & datetime & "'"
	set command to command & " +%s"
	return do shell script command
end timestamp

tell application "Music"
	set sel to selection

	repeat with outerTrack in sel
		tell outerTrack to set {da} to {date added, track number, disc number, album}
		set innerTrack to contents of outerTrack
		set comment of the innerTrack to my timestamp(da)
	end repeat

end tell
