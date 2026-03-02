if [[ -o interactive ]]; then
    autoload -Uz add-zsh-hook

    typeset -g fahhh_last_cmd=""

    fahhh_handler() {
        nohup ::FAHHH:: play >/dev/null 2>&1 </dev/null &
        disown 2>/dev/null
    }

    fahhh_preexec() {
        fahhh_last_cmd="$1"
    }

    fahhh_precmd() {
        local exit_code=$?
        if [[ -n "${fahhh_last_cmd}" && $exit_code -ne 0 ]]; then
            fahhh_handler
        fi
        fahhh_last_cmd=""
    }

    add-zsh-hook preexec fahhh_preexec
    add-zsh-hook precmd fahhh_precmd
fi
