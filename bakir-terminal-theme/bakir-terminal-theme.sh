#!/bin/bash
# --- Bakir-Linux Theme Engine ---
CYN='\e[1;38;5;51m'
GLD='\e[1;38;5;220m'
RED='\e[1;31m'
NC='\e[0m'
USER_NAME=$(whoami)

clear
echo -e "${CYN}┌──────────────────────────────────────────┐${NC}"
echo -e "${CYN}│${NC}      ${GLD}Bakir-Linux Neon Theme${NC}            ${CYN}│${NC}"
echo -e "${CYN}└──────────────────────────────────────────┘${NC}"
echo -e ""
echo -e "${GLD}يتم الآن تفعيل الهوية البصرية لـ ${CYN}$USER_NAME${NC}"

cat << 'USEREOF' > ~/.bashrc
# --- Bakir-Linux Neon Theme ---
export HISTCONTROL=ignoreboth
shopt -s checkwinsize
C_CYN='\[\e[1;38;5;51m\]'
C_GLD='\[\e[1;38;5;220m\]'
C_RED='\[\e[1;31m\]'
S_2='  '
PS1="\[\${C_CYN}\]┌──\[\${C_GLD}\](\[\${C_CYN}\]\u\[\${C_GLD}\])\[\${S_2}\]\[\${C_GLD}\]» \[\${C_RED}\]B \[\${C_GLD}\]«\[\${S_2}\]\[\${C_GLD}\](\[\${C_CYN}\]Bakir-Linux\[\${C_GLD}\])\n\[\${C_CYN}\]└─\[\${C_GLD}\][\[\${C_CYN}\]\w\[\${C_GLD}\]] » \[\${C_CYN}\]"
export LS_COLORS="di=1;38;5;51:ln=1;38;5;220:ex=1;38;5;82"
alias ls='ls --color=auto'
trap 'echo -ne "\e[1;38;5;51m"' DEBUG
USEREOF

sudo bash -c "cat << 'ROOTEOF' > /root/.bashrc
# --- Bakir-Linux Root Identity ---
export HISTCONTROL=ignoreboth
shopt -s checkwinsize
C_RED='\[\e[1;31m\]'
C_CYN='\[\e[1;38;5;51m\]'
C_GLD='\[\e[1;38;5;220m\]'
S_2='  '
PS1=\"\[\${C_RED}\]┌──\[\${C_GLD}\](\[\${C_RED}\]\u\[\${C_GLD}\])\[\${S_2}\]\[\${C_GLD}\]» \[\${C_CYN}\]B \[\${C_GLD}\]«\[\${S_2}\]\[\${C_GLD}\](\[\${C_RED}\]Bakir-Linux-ROOT\[\${C_GLD}\])\n\[\${C_RED}\]└─\[\${C_GLD}\][\[\${C_RED}\]\w\[\${C_GLD}\]] » \[\${C_RED}\]\"
export LS_COLORS='di=1;31:ln=1;33:ex=1;32'
alias ls='ls --color=auto'
trap 'echo -ne \"\e[1;31m\"' DEBUG
ROOTEOF"

echo -e "${CYN}[✔] تم حقن الثيم للمستخدم والروت بنجاح!${NC}"
echo -e "${GLD}أعد فتح الترمينال لتشهد القوة الفيروزية!${NC}"
