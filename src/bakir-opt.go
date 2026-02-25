package main

import (
	"fmt"
	"os"
	"os/exec"
)

func main() {
	if os.Geteuid() != 0 {
		fmt.Println("❌ خطأ سيادي: يجب تشغيل Bakir-Opt بصلاحيات sudo")
		return
	}

	fmt.Println("🛡️ Bakir-Opt Ultimate v5.0 | نظام الصيانة والفرض السيادي")
	fmt.Println("--------------------------------------------------")

	// المرحلة 0: فرض وجود الأدوات (علاج مشكلة command not found)
	fmt.Println("📦 المرحلة 0: تأمين أدوات الفحص والنظام...")
	executeReal("apt-get update && apt-get install -y zram-tools procps util-linux findutils")

	// المرحلة 1: الصيانة العلاجية الحقيقية (إصلاح الحزم)
	fmt.Println("🔍 المرحلة 1: فحص وإصلاح أخطاء النظام والحزم...")
	executeReal("dpkg --configure -a")           
	executeReal("apt-get install -f -y")        

	// المرحلة 2: التطهير العميق
	fmt.Println("🧹 المرحلة 2: التطهير العميق وإزالة الحزم المعطلة...")
	executeReal("apt-get autoremove -y")
	executeReal("apt-get autoclean -y")

	// المرحلة 3: تفعيل ZRAM وتحسين الأداء (فعل حقيقي)
	fmt.Println("⚡ المرحلة 3: تحسين الأداء (ZRAM & SSD Trim)...")
	executeReal("modprobe zram || true")
	executeReal("zramctl --find --size 2G || true")
	executeReal("fstrim -av")

	// المرحلة 4: فرض إعدادات الشبكة (TCP BBR)
	fmt.Println("🌐 المرحلة 4: تحسين استجابة الشبكة (TCP BBR)...")
	executeReal("echo 10 | tee /proc/sys/vm/swappiness")
	executeReal("echo 'net.core.default_qdisc=fq' | tee -a /etc/sysctl.conf")
	executeReal("echo 'net.ipv4.tcp_congestion_control=bbr' | tee -a /etc/sysctl.conf")
	executeReal("sysctl -p")

	fmt.Println("--------------------------------------------------")
	fmt.Println("✅ تم الانتهاء! نظام باكير الآن محمي ومحسّن فعلياً وبأدوات حقيقية.")
}

func executeReal(command string) {
	cmd := exec.Command("sh", "-c", command)
	cmd.Stdout = os.Stdout // ربط المخرج لترا الحقيقة بعينك
	cmd.Stderr = os.Stderr 
	cmd.Run()
}
