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

	fmt.Println("🛡️ Bakir-Opt Ultimate v4.0 | نظام الصيانة والتحسين الشامل")
	fmt.Println("--------------------------------------------------")

	// 1. فحص وإصلاح الحزم (الجانب العلاجي)
	fmt.Println("🔍 المرحلة 1: فحص وإصلاح أخطاء النظام والحزم...")
	execute("dpkg --configure -a")           
	execute("apt-get install -f -y")        
	execute("apt-get update --fix-missing") 

	// 2. تنظيف الأخطاء والمخلفات العميق
	fmt.Println("🧹 المرحلة 2: التطهير العميق وإزالة الحزم اليتيمة...")
	execute("apt-get autoremove -y")
	execute("apt-get autoclean -y")
	execute("apt-get clean")

	// 3. تحسين أداء القرص والنواة
	fmt.Println("⚡ المرحلة 3: تحسين الأداء (ZRAM & SSD Trim)...")
	run("modprobe", "zram")
	run("zramctl", "--find", "--size", "2G")
	run("fstrim", "-av")

	// 4. ضبط ذكاء الشبكة والنواة
	fmt.Println("🌐 المرحلة 4: تحسين استجابة الشبكة (TCP BBR)...")
	execute("echo 10 | tee /proc/sys/vm/swappiness")
	execute("echo 'net.core.default_qdisc=fq' | tee -a /etc/sysctl.conf")
	execute("echo 'net.ipv4.tcp_congestion_control=bbr' | tee -a /etc/sysctl.conf")
	run("sysctl", "-p")

	fmt.Println("--------------------------------------------------")
	fmt.Println("✅ تم الانتهاء! نظام باكير الآن محمي، نظيف، ومحسّن بالكامل.")
}

func run(name string, args ...string) {
	cmd := exec.Command(name, args...)
	cmd.Run()
}

func execute(command string) {
	cmd := exec.Command("sh", "-c", command)
	cmd.Run()
}
