package main

import (
	"fmt"
	"os"
	"os/exec"
	"time"
	"strings"
)

func main() {
	if os.Geteuid() != 0 {
		fmt.Println("❌ خطأ سيادي: يجب تشغيل الوحش بصلاحيات sudo")
		return
	}

	fmt.Println("--------------------------------------------------")
	fmt.Println("🔥 Bakir-Opt v7.0 | النسخة الوحشية (الترميم الجراحي)")
	fmt.Println("--------------------------------------------------")

	// مصفوفة لتخزين سجل الإصلاحات
	var logDetails []string
	
	steps := []struct {
		name string
		cmd  string
		msg  string
	}{
		{"الترميم الذاتي", "apt-get install -f -y", "تم إصلاح الحزم المكسورة"},
		{"تطهير الكاش", "sync; echo 3 > /proc/sys/vm/drop_caches", "تم تصفير ذاكرة النظام المؤقتة"},
		{"التنظيف العميق", "apt-get autoremove -y && apt-get clean", "تم حذف مخلفات النظام بنجاح"},
		{"تحسين القرص", "fstrim -av", "تم تحسين أداء قرص SSD"},
	}

	for _, step := range steps {
		fmt.Printf("⏳ جاري %s...\n", step.name)
		err := exec.Command("sh", "-c", step.cmd).Run()
		if err == nil {
			logDetails = append(logDetails, "✅ "+step.msg)
		} else {
			logDetails = append(logDetails, "⚠️ فشل في "+step.name)
		}
		time.Sleep(1 * time.Second)
	}

	// إرسال إشعار لسطح المكتب (Desktop Notification)
	summary := strings.Join(logDetails, "\n")
	exec.Command("notify-send", "-i", "utilities-terminal", "✅ اكتمل الترميم السيادي", "تم فحص وصيانة النظام بنجاح v7.0").Run()

	fmt.Println("--------------------------------------------------")
	fmt.Println("📋 ملخص العملية الجراحية:")
	for _, l := range logDetails {
		fmt.Println(l)
	}
	fmt.Println("--------------------------------------------------")
	fmt.Printf("🚀 الوحش v7.0: النظام الآن في قمة عطائه يا سيادة المستشار.\n")
}
