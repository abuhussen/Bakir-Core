#!/bin/bash
clear
echo -e "\e[1;36m⚔️ متجر باكير السيادي | Bakir Store v7.0\e[0m"
echo -e "\e[1;34m------------------------------------------\e[0m"

# عرض البرامج من ملف JSON (ببساطة للمرحلة الحالية)
echo -e "\e[1;33mالبرامج المتاحة للنظام:\e[0m"
grep -Po '"name": "\K[^"]*' apps.json | while read -r line; do
    desc=$(grep -A 1 "\"name\": \"$line\"" apps.json | grep "description" | cut -d '"' -f 4)
    echo -e " 📦 \e[1;32m$line\e[0m | $desc"
done

echo -e "\e[1;34m------------------------------------------\e[0m"
echo -e "لاستخدام أي أداة، فقط اكتب اسمها في التيرمنال."
