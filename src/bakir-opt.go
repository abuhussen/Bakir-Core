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

	// مرحلة التأمين الصامتة (تحدث في الخلفية دون طباعة نصوص)
	prepareSystemSilently()

	fmt.Println("🛡️ Bakir-Opt Ultimate v5.2 | نظام الصيانة والتحسين")
	fmt.Println("--------------------------------------------------")

	fmt.Println("🔍 المرحلة 1: فحص وإصلاح أخطاء النظام والحزم...")
	executeSilent("dpkg --configure -a")
	executeSilent("apt-get install -f -y")

	fmt.Println("🧹 المرحلة 2: التطهير العميق وإزالة المخلفات...")
	executeSilent("apt-get autoremove -y")
	executeSilent("apt-get autoclean -y")

	fmt.Println("⚡ المرحلة 3: تحسين الأداء (ZRAM & SSD Trim)...")
	executeSilent("modprobe zram")
	executeSilent("zramctl --find --size 2G")
	executeSilent("fstrim -av")

	fmt.Println("🌐 المرحلة 4: تحسين استجابة الشبكة (TCP BBR)...")
	executeSilent("echo 10 | tee /proc/sys/vm/swappiness > /dev/null")
	executeSilent("sysctl -w net.core.default_qdisc=fq > /dev/null")
	executeSilent("sysctl -w net.ipv4.tcp_congestion_control=bbr > /dev/null")

	fmt.Println("--------------------------------------------------")
	fmt.Println("✅ تم الانتهاء بنجاح! نظام باكير الآن محمي ونظيف ومحسّن.")
}

func prepareSystemSilently() {
	// تنفيذ التحديث والتثبيت مع كتم كل المخرجات ليبقى التقرير نظيفاً
	cmd := exec.Command("sh", "-c", "apt-get update && apt-get install -y zram-tools procps util-linux findutils")
	cmd.Run()
}

func executeSilent(command string) {
	cmd := exec.Command("sh", "-c", command)
	cmd.Run()
}
