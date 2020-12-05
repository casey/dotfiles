property my_title : "Clear Work"

tell application "iTunes"
	set sel to selection

	repeat with outerTrack in sel
		set innerTrack to contents of outerTrack
		set work of innerTrack to ""
	end
end
