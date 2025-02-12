# Widget function to find and insert npm commands
function _npm_runzf() {
    # Use our Rust tool to get the commands
    local selected=$(npm-runzf | fzf \
        --height=40% \
        --reverse \
        --ansi \
        --preview 'echo "Command: {1}\nWorkspace: {2}"' \
        --preview-window=up:2:wrap \
        --bind 'ctrl-/:toggle-preview')

    if [[ -n "$selected" ]]; then
        local cmd=$(echo "$selected" | cut -f1)
        local ws=$(echo "$selected" | cut -f2)
        
        # Clear the current line
        LBUFFER=""
        RBUFFER=""
        
        if [[ -n "$ws" ]]; then
            LBUFFER="npm run -w $ws $cmd"
        else
            LBUFFER="npm run $cmd"
        fi
    fi
    
    zle reset-prompt
}

# Create the widget
zle -N _npm_runzf

bindkey '†' _npm_runzf
