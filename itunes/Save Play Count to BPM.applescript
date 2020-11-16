property my_title : "Save Play Count to BPM"

tell application "iTunes"
	set sel to selection
	repeat with t in sel
    set plays to t's «class pPlC»
    set t's bpm to plays
	end repeat
end tell
