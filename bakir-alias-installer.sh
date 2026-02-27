#!/bin/bash
# Bakir Linux Alias Installer - الدستور البرمجي
ALIAS_FILE="$HOME/.bash_aliases"
BASHRC_FILE="$HOME/.bashrc"

echo "🦁 جاري تثبيت الاختصارات السيادية..."
cat << 'EOT' > $ALIAS_FILE
# --- BAKIR LINUX SOVEREIGN ALIASES ---
alias update='sudo apt update'
alias upgrade='sudo apt upgrade'
alias install='sudo apt install'
alias reboot='sudo reboot'
alias md='mkdir -p'
alias cp='cp -r'
alias rm='rm -rf'
alias cls='clear'
alias store='bakir-store'
alias shield='sudo bakir-shield'
alias opt='sudo bakir-opt'
alias sys='bakir-sys'
alias get='bakir-get'
alias get-v='bakir-get -v'
alias get-a='bakir-get -a'
alias get-s='bakir-get -s'
alias ..='cd ..'
alias ...='cd ../..'
alias proj='cd ~/Bakir-Project'
alias core='cd ~/Bakir-Core'
alias zip='tar -czvf'
alias myip='hostname -I'
alias reload='source ~/.bashrc && source ~/.bash_aliases'
EOT

if ! grep -q ".bash_aliases" "$BASHRC_FILE"; then
    echo "if [ -f ~/.bash_aliases ]; then . ~/.bash_aliases; fi" >> "$BASHRC_FILE"
fi
source ~/.bashrc
echo "🎯 تمت العملية! الأوامر الآن فعالة."
