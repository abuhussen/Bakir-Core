#!/bin/bash
# سكربت لتثبيت أدوات Bakir Linux دفعة واحدة

echo "جاري تحميل وتثبيت أدوات Bakir..."

# قائمة بأسماء البرامج الخمسة (تأكد أن الأسماء مطابقة لملفات الـ deb في المستودع)
PACKAGES=("bakir-shield.deb" "bakir-gost.deb" "bakir-guard.deb" "bakir-redbutton.deb" "bakir-status.deb")

# تحميل البرامج
for pkg in "${PACKAGES[@]}"; do
    echo "تحميل $pkg..."
    wget -q https://github.com/abuhussen/Bakir-Core/raw/main/$pkg
done

# تثبيت البرامج
echo "جاري التثبيت..."
sudo apt install -y ./*.deb

# تنظيف الملفات بعد التثبيت
rm *.deb
echo "تم تثبيت جميع الأدوات بنجاح!"
