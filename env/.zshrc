# ============================================================================
# Random Setup
# ============================================================================
[ -f "/home/kairos/.ghcup/env" ] && . "/home/kairos/.ghcup/env" # ghcup-env

export XDG_CONFIG_HOME=$HOME/.config
export VIM='nvim'
export EDITOR='nvim'
export PAGER='less'
export LSCOLORS='ExGxbEaECxxEhEhBaDaCaD'

# Tmux-Sessionizer made by the primeagen is incredible
bindkey -s ^f "tmux-sessionizer\n"

alias gs="git status"
alias woman="man"
alias ':e'="nvim ."
alias ':q'="exit"
alias 'today'='date +"%A %d/%m/%Y %H:%M:%S %Z (%:z)"'

# Stole these commands from ZSH
alias -='cd -'
alias ..='echo "cd .."; cd ..'
alias ...=../..
alias ....=../../..
alias .....=../../../..
alias ......=../../../../..

# Try to alias grep to use colours
grep --color=auto < /dev/null &>/dev/null &&
    alias grep='grep --color=auto'

alias l='ls -lah'
alias la='ls -lAh'
alias ll='ls -lh'
alias ls='ls --color=tty'
alias lsa='ls -lah'

# Enable color support of ls
if ls --color=auto &>/dev/null; then
    alias ls='ls -p --color=auto'
else
    alias ls='ls -p -G'
fi

# I have no idea if this works but something something fzf
[ -f ~/.fzf.zsh ] && source ~/.fzf.zsh

shopt -s cdspell
shopt -s checkwinsize
shopt -s extglob

shopt -s autocd   2>/dev/null || true
shopt -s dirspell 2>/dev/null || true

# ============================================================================
# Terminal Styling
# ============================================================================
# manpage styling, stolen from Dave Eddy (https://github.com/bahamas10/dotfiles)
export LESS_TERMCAP_mb=$'\e[1;31m'
export LESS_TERMCAP_md=$'\e[1;31m'
export LESS_TERMCAP_me=$'\e[0m'
export LESS_TERMCAP_se=$'\e[0m'
export LESS_TERMCAP_so=$'\e[1;33;44m'
export LESS_TERMCAP_ue=$'\e[0m'
export LESS_TERMCAP_us=$'\e[4;1;32m'
export LESS_TERMCAP_mr=$'\e[7m'
export LESS_TERMCAP_mh=$'\e[2m'
export LESS_TERMCAP_ZN=$'\e[74m'
export LESS_TERMCAP_ZV=$'\e[75m'
export LESS_TERMCAP_ZO=$'\e[73m'
export LESS_TERMCAP_ZW=$'\e[75m'
export MANPAGER='less'

# Shamelessly stolen from Dave Eddy  (https://github.com/bahamas10/dotfiles)
COLOR256=()
COLORS256[0]=$(tput setaf 1)
COLORS256[256]=$(tput sgr0)
COLORS256[257]=$(tput bold)

PROMPT_COLORS=()
set_prompt_colors() {
    local h=${1:-0}
    local color=
    local i=0
    local j=0
    for i in {22..231}; do
        ((i % 30 == h)) || continue

        color=${COLOR256[$i]}
        # cache the tput colors
        if [[ -z $color ]]; then
            COLOR256[$i]=$(tput setaf "$i")
            color=${COLOR256[$i]}
        fi
        PROMPT_COLORS[$j]=$color
        ((j++))
    done
}

# [(exit code)] <hostname> <uname> <cwd> [<git branch>]] $

# Exit code of last process
PS1='$(ret=$?;(($ret!=0)) && echo "\[${COLOR256[0]}\]($ret) \[${COLOR256[256]}\]")'

# hostname
PS1+='\[${PROMPT_COLORS[3]}\]\h '

# uname
PS1+='\[${PROMPT_COLORS[2]}\]'"$(uname | tr '[:upper:]' '[:lower:]')"' '

# cwd
PS1+='\[${PROMPT_COLORS[5]}\]\w '

# optional git branch
PS1+='$(branch=$(git rev-parse --abbrev-ref HEAD 2>/dev/null); [[ -n $branch ]] && echo "\[${PROMPT_COLORS[2]}\](\[${PROMPT_COLORS[3]}\]git:$branch\[${PROMPT_COLORS[2]}\]) ")'

# Prompt Character
PS1+='\[${PROMPT_COLORS[0]}\]\$\[${COLOR256[256]}\] '

set_prompt_colors_24
# ============================================================================
# Commands
# ============================================================================

# .zsh_profile holds all of my commands
source ~/.zsh_profile

# ============================================================================
# Path Setup
# ============================================================================

addToPathFront $HOME/.local/scripts
addToPathFront $HOME/.local/bin
#addToPathFront $HOME/.local/npm/bin
#addToPath $HOME/.local/go/bin
#addToPath $HOME/kaas/tmux-sessionizer
addToPath $HOME/opt/gf/bin

