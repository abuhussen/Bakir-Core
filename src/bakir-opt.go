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

	fmt.Println("⚡ Bakir-Opt v3.1 | المحرك القتالي لنظام باكير")
	fmt.Println("--------------------------------------------------")

	// 1. ZRAM
	fmt.Println("🚀 تفعيل تقنية ZRAM (مضاعفة كفاءة الرام)...")
	run("modprobe", "zram")
	run("zramctl", "--find", "--size", "2G")

	// 2. SSD Trim
	fmt.Println("💾 تحسين أداء القرص الصلب (SSD Optimization)...")
	run("fstrim", "-av")

	// 3. Kernel Swappiness (استخدام bash -c مع sudo tee لتجنب خطأ الصلاحيات)
	fmt.Println("🧠 ضبط ذكاء النواة (Swappiness Tuning)...")
	execute("echo 10 | tee /proc/sys/vm/swappiness")

	// 4. TCP BBR & Network Optimization
	fmt.Println("🌐 تسريع استجابة الشبكة (TCP BBR)...")
	execute("echo 'net.core.default_qdisc=fq' | tee -a /etc/sysctl.conf")
	execute("echo 'net.ipv4.tcp_congestion_control=bbr' | tee -a /etc/sysctl.conf")
	run("sysctl", "-p")

	// 5. التنظيف العميق
	fmt.Println("🧹 جاري كنس مخلفات النظام...")
	run("apt-get", "autoremove", "-y")
	run("apt-get", "autoclean", "-y")
	run("find", "/tmp", "-type", "f", "-atime", "+1", "-delete")

	fmt.Println("--------------------------------------------------")
	fmt.Println("✅ اكتملت العملية! نظام Bakir Linux الآن في قمة نشاطه.")
}

func run(name string, args ...string) {
	cmd := exec.Command(name, args...)
	cmd.Run()
}

// دالة تنفيذ الأوامر المعقدة التي تحتاج لـ Pipe (|) و Tee
func execute(command string) {
	cmd := exec.Command("sh", "-c", command)
	cmd.Run()
}
