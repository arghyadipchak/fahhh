set -E

fahhh_handler() {
    nohup ::FAHHH:: play >/dev/null 2>&1 </dev/null &
    disown 2>/dev/null
}

trap 'fahhh_handler' ERR
