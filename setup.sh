#!/bin/bash

# 1. إضافة مفتاح المستودع الخاص بك لكي يثق النظام في برامجك
echo "Adding Bakir Linux Repository Key..."
curl -s https://raw.githubusercontent.com/abuhussen/Bakir-Core/main/public.key | sudo gpg --dearmor -o /usr/share/keyrings/bakir-archive-keyring.gpg

# 2. إضافة المستودع إلى قائمة مصادر النظام
echo "Adding Repository to sources.list.d..."
echo "deb [signed-by=/usr/share/keyrings/bakir-archive-keyring.gpg] https://raw.githubusercontent.com/abuhussen/Bakir-Core/main/ dists/stable/main/" | sudo tee /etc/apt/sources.list.d/bakir.list > /dev/null

# 3. تحديث قائمة الحزم
echo "Updating apt cache..."
sudo apt update

echo "Done! You can now install your tools using: sudo apt install <package-name>"
