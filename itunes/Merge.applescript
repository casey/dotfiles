property my_title : "Merge"

to split(s, d)
  set o to AppleScript's text item delimiters
  set AppleScript's text item delimiters to d
  set l to s's every text item
  set AppleScript's text item delimiters to o
  return l
end

to sort(l)
  repeat with i from 1 to (count of l) - 1
    repeat with j from i + 1 to count of l
      set i_track to (item i of l)'s track number
      set j_track to (item j of l)'s track number
      if i_track > j_track then
        set temp to item i of l
        set item i of l to item j of l
        set item j of l to temp
      end
    end
  end
end

to dupes(l)
  repeat with i from 1 to (count of l) - 1
    repeat with j from i + 1 to count of l
      set i_track to (item i of l)'s track number
      set j_track to (item j of l)'s track number
      if i_track = j_track
        my cancel("Duplicate track number: " & i_track)
      end
    end
  end
end

to trash_file(l)
  do shell script "mv " & quoted form of POSIX path of (l as string) & " " & quoted form of POSIX path of (path to trash as string)
end

to cancel(m)
  display dialog m
  error number -128
end

tell application "iTunes"
  set n to {}
  set o to {}

  repeat with t in selection
    set p to t's location as string
    set f to my split(p, ":")'s last item
    set i to my split(f, ".")'s first item as number
    if i ≥ 9172 and i ≤ 19961
      copy t to o's end
    else
      copy t to n's end
    end
  end

  if n's length ≠ o's length
    my cancel("Item count mismatch: " & n's length & " ≠ " & o's length)
  end

  if n's length = 0
    my cancel("Selection is empty.")
  end

  my sort(n)
  my sort(o)
  my dupes(n)
  my dupes(o)

  set message to ""
  repeat with i from 1 to count of n
    set old to item i of o
    set new to item i of n
    set old_track to old's track number
    set new_track to new's track number
    if old_track ≠ new_track
      my cancel("Mismatched track numbers: " & old_track & " ≠ " & new_track)
    end

    set old_name to old's name
    set new_name to new's name

    set message to message & old_track & ".	" & old_name & "\n"

    if new_name ≠ old_name
      set message to message & "	" & new_name & "\n"
    end

  end
  display dialog message

  set normal to {}
  repeat with p in playlists
    if p's visible and p's special kind = none and not p's genius and not p's smart and index of last track of p < 1500
      set normal to normal & {p}
    end
  end

  repeat with i from 1 to count of n
    set old to item i of o
    set new to item i of n
    set oldid to old's persistent ID

    repeat with j from 1 to count of normal
      set p to item j of normal
      set ts to tracks of p
      repeat with k from 1 to count of ts
        set t to item k of ts
        if t's persistent ID = oldid
          delete t
          duplicate new to p
        end
      end
    end
  end

  set old_database_ids to {}
  repeat with i from 1 to count of n
    set old to item i of o
    set new to item i of n
    set old_plays to old's «class pPlC»
    set new_plays to new's «class pPlC»
    set old_dbid to old's database ID
    set new's «class pPlC» to old_plays + new_plays
    set old's «class pPlC» to 0
    my trash_file(location of old)
    set end of old_database_ids to old_dbid
  end

  repeat with old_database_id in old_database_ids
    delete (some track of library playlist 1 whose database ID is old_database_id)
  end
end
