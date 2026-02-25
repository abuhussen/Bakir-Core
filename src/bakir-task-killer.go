package main

import (
	"fmt"
	"os"
	"os/exec"
	"strings"
)

func main() {
	if os.Geteuid() != 0 {
		fmt.Println("❌ خطأ: هذه الأداة السيادية تتطلب صلاحيات sudo لإنهاء العمليات.")
		return
	}

	fmt.Println("⚔️ Bakir-Task-Killer v1.0 | نظام التطهير البرمجي")
	fmt.Println("--------------------------------------------------")
	fmt.Println("1. إنهاء العمليات المعلقة (Not Responding)")
	fmt.Println("2. تصفية العمليات المستهلكة للرام (> 1GB)")
	fmt.Println("3. إنهاء برنامج معين باسمه")
	fmt.Println("--------------------------------------------------")
	fmt.Print("اختر الإجراء المطلوب: ")

	var choice int
	fmt.Scanln(&choice)

	switch choice {
	case 1:
		fmt.Println("🔍 جاري البحث عن العمليات المتجمدة...")
		// في لينكس العمليات المعلقة غالبا تكون في حالة Zombie أو Uninterruptible sleep
		cmd := exec.Command("sh", "-c", "ps -eo state,pid,cmd | grep '^Z' | awk '{print $2}' | xargs -r kill -9")
		cmd.Run()
		fmt.Println("✅ تم تطهير النظام من العمليات الميتة.")
	case 2:
		fmt.Println("🧠 جاري فحص استهلاك الرام...")
		// البحث عن العمليات التي تستهلك أكثر من 1 جيجا (تقريبياً)
		cmd := exec.Command("sh", "-c", "ps -eo pmem,pid,cmd | awk '$1 > 10.0 {print $2}' | xargs -r kill -9")
		cmd.Run()
		fmt.Println("✅ تم إنهاء البرامج التي كانت تهدد استقرار الذاكرة.")
	case 3:
		fmt.Print("✍️ أدخل اسم البرنامج (مثلاً firefox): ")
		var procName string
		fmt.Scanln(&procName)
		exec.Command("pkill", "-9", procName).Run()
		fmt.Printf("🎯 تم إنهاء كل عمليات %s فوراً.\n", procName)
	default:
		fmt.Println("❌ اختيار غير صحيح.")
	}
}
