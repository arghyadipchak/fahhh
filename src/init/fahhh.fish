function fahhh_handler
    nohup ::FAHHH:: play >/dev/null 2>&1 </dev/null &
    disown 2>/dev/null
end

function fahhh_postexec --on-event fish_postexec
    if test $status -ne 0
        fahhh_handler
    end
end
