property my_title : "Clear Show"

tell application "Music"
	set sel to selection

	repeat with outerTrack in sel
		set innerTrack to contents of outerTrack
		set show of innerTrack to ""
	end
end
