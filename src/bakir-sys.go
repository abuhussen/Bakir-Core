package main

import (
	"fmt"
	"os/exec"
	"strings"
)

func main() {
	fmt.Println("📊 Bakir-Sys v1.0 | مراقب النظام السيادي")
	fmt.Println("--------------------------------------------------")

	// 1. معلومات المعالج
	cpu, _ := exec.Command("sh", "-c", "grep 'model name' /proc/cpuinfo | head -1 | cut -d: -f2").Output()
	fmt.Printf("💻 المعالج: %s", strings.TrimSpace(string(cpu)))

	// 2. استخدام الرام
	mem, _ := exec.Command("free", "-h").Output()
	memLines := strings.Split(string(mem), "\n")
	fmt.Printf("\n🧠 الذاكرة (RAM): %s", strings.Fields(memLines[1])[2] + " / " + strings.Fields(memLines[1])[1])

	// 3. مساحة الهاردسك (القسم الرئيسي)
	disk, _ := exec.Command("df", "-h", "/").Output()
	diskLines := strings.Split(string(disk), "\n")
	fmt.Printf("\n💽 القرص الصلب: %s", strings.Fields(diskLines[1])[2] + " مستخدم من أصل " + strings.Fields(diskLines[1])[1])

	// 4. درجة الحرارة (إذا توفرت الحساسات)
	temp, _ := exec.Command("sh", "-c", "vcgencmd measure_temp 2>/dev/null || sensors 2>/dev/null | grep 'Package id 0' | awk '{print $4}'").Output()
	if len(temp) > 0 {
		fmt.Printf("\n🌡️ درجة الحرارة: %s", strings.TrimSpace(string(temp)))
	}

	// 5. وقت تشغيل النظام
	uptime, _ := exec.Command("uptime", "-p").Output()
	fmt.Printf("\n⏱️ مدة التشغيل: %s", strings.TrimSpace(string(uptime)))

	fmt.Println("\n--------------------------------------------------")
	fmt.Println("✅ تم فحص النظام بنجاح يا سيادة المستشار.")
}
