property my_title : "Group"

tell application "Music"
	set sel to selection

	repeat with outerTrack in sel
		tell outerTrack to set {da, tn, dn, a} to {date added, track number, disc number, album}
		set innerTrack to contents of outerTrack
		set y to (do shell script "echo " & date string of da & " | grep -o '[0-9]\\{4\\}'")
    set m to ((month of da) as integer)
    set d to day of da
    set di to 10 - dn
    set ti to 100 - tn
		set g to (y & "." &  m & "." & d & "." & a & "." & di & "." & ti)
		set show of innerTrack to g
	end

end
