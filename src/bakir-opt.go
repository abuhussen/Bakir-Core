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

	// ملخص العملية
	summaryText := strings.Join(logDetails, "\n")
	
	// إرسال الإشعار لسطح المكتب حتى مع استخدام sudo
	currentUser := os.Getenv("SUDO_USER")
	if currentUser == "" {
		currentUser = os.Getenv("USER")
	}
	
	// أمر الإشعار المتقدم
	notificationCmd := fmt.Sprintf("sudo -u %s DISPLAY=:0 DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/$(id -u %s)/bus notify-send -i utilities-terminal '✅ اكتمل الترميم السيادي' 'تم صيانة نظام باكير بنجاح'", currentUser, currentUser)
	exec.Command("sh", "-c", notificationCmd).Run()

	fmt.Println("--------------------------------------------------")
	fmt.Println("📋 ملخص العملية الجراحية:")
	fmt.Println(summaryText)
	fmt.Println("--------------------------------------------------")
	fmt.Printf("🚀 الوحش v7.0: النظام الآن في قمة عطائه يا سيادة المستشار.\n")
}
