tell application "Swinsian"

	(*set selected to the selected tracks in swinsian*)
	set selected to selection of window 1
	repeat with t in selected
		log ("")
		log ("Track: " & name of t)

		-- Get the path of the track, and quote it
		log ("	Path:                    " & (POSIX path of (get location of t)))
		set pp to (POSIX path of (get location of t))
		set qpath to quoted form of pp

		-- call the wrapper script for the comment
		set scr_comm to "../wrapper.sh " & qpath
		if comment of t is missing value then
			log ("	Comment: no comment in track")
			set udat to ""
			set scr_comm to scr_comm & ""
		else
			log ("	Existing Comment:        " & comment of t)
			set com to comment of t
			set qcomment to quoted form of com
			set scr_comm to scr_comm & " " & qcomment
		end if

		-- Call the wrapper script
		log ("	Script call (comment):   " & scr_comm)
		set script_result_comm to do shell script scr_comm
		log ("	Script result (comment): " & script_result_comm)

		-- set the comment of the track
		set the comment of t to script_result_comm


		-- get the path again, as it might have moved.
		log ("	Path:                    " & (POSIX path of (get location of t)))
		set pp to (POSIX path of (get location of t))
		set qpath to quoted form of pp

		-- call the wrapper script for the name
		set scr_nam to "../wrapper.sh " & qpath
		set nam to name of t
		set qnam to quoted form of nam
		set scr_nam to scr_nam & " " & qnam

		-- Call the wrapper script
		log ("	Script call (title):     " & scr_nam)
		set script_result_nam to do shell script scr_nam
		log ("	Script result (title):   " & script_result_nam)

		-- set the name of the track
		set the name of t to script_result_nam
	end repeat
end tell