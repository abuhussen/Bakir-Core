package main

import (
	"fmt"
	"os"
)

func main() {
	fmt.Println("🏪 متجر باكير السيادي (Bakir Store) | الإصدار 2.6")
	fmt.Println("--------------------------------------------------")
	
	if len(os.Args) < 2 {
		displayMenu()
		return
	}

	arg := os.Args[1]
	switch arg {
	case "list":
		displayMenu()
	case "update":
		fmt.Println("🔄 جاري مزامنة المستودعات السيادية مع GitHub...")
	default:
		fmt.Println("❌ خيار غير معروف. استخدم 'bakir-store list' لعرض كافة الأقسام.")
	}
}

func displayMenu() {
	fmt.Println("🇸🇾 [القسم الأول: البرامج السيادية - Bakir Core Tools]")
	fmt.Println("1. bakir-get           📥 (مدير الحزم الصاروخي - يشمل yt-dlp و aria2)")
	fmt.Println("2. bakir-shield        🛡️  (نظام الحماية والجدار الناري)")
	fmt.Println("3. bakir-opt           ⚡ (أداة تحسين وتسريع النظام)")
	fmt.Println("4. bakir-terminal-theme 🎨 (تخصيص مظهر التيرمنال)")
	fmt.Println("5. bakir-store         🏪 (المتجر المركزي)")

	fmt.Println("\n🌍 [القسم الثاني: البرامج العالمية الأساسية - Global Tools]")
	fmt.Println("1. firefox-esr         🌐 (متصفح الإنترنت الرسمي والآمن)")
	fmt.Println("2. timeshift           ⏳ (آلة الزمن لاستعادة النظام عند الانهيار)")
	fmt.Println("3. vlc                 🎬 (مشغل الوسائط المتعددة الشامل)")
	fmt.Println("4. gparted             🗂️  (أداة إدارة وتقسيم الأقراص)")
	fmt.Println("5. libnotify-bin       🔔 (نظام تنبيهات النظام)")
	fmt.Println("6. htop                📊 (مراقب موارد النظام المتقدم)")

	fmt.Println("\n✨ [قيد التطوير: Bakir Themes & Icons Project]")
	fmt.Println("--------------------------------------------------")
}
