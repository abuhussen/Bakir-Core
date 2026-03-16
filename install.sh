#!/bin/bash

# التحقق من أن المستخدم يملك صلاحيات الجذر (root)
if [ "$EUID" -ne 0 ]; then
  echo "يرجى تشغيل هذا السكربت باستخدام sudo"
  exit
fi

echo "جاري تثبيت أدوات Bakir..."

# نسخ الملفات التنفيذية إلى مسار النظام
cp bakir-shield bakir-git bakir-opt bakir-t-t bakir-alias /usr/local/bin/

# إعطاء صلاحيات التشغيل للملفات
chmod +x /usr/local/bin/bakir-*

echo "تم التثبيت بنجاح! يمكنك الآن استخدام الأدوات مباشرة من الترمينال."
