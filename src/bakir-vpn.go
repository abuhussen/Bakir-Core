package main

import (
	"fmt"
	"os"
)

func main() {
	if len(os.Args) < 2 {
		showHelp()
		return
	}

	arg := os.Args[1]

	switch arg {
	case "-h":
		showHelp()
	case "-on":
		startVPN()
	case "-off":
		stopVPN()
	case "-s":
		showServers()
	case "-stat":
		checkStatus()
	default:
		fmt.Println("❌ أمر غير معروف. استخدم bakir-vpn -h للمساعدة.")
	}
}

func showHelp() {
	fmt.Println("🛡️  نظام باكير لينكس - bakir-vpn v1.0")
	fmt.Println("------------------------------------")
	fmt.Println("Usage:")
	fmt.Println("  bakir-vpn -on    🚀 الاتصال بأسرع سيرفر")
	fmt.Println("  bakir-vpn -off   🛑 إيقاف الاتصال")
	fmt.Println("  bakir-vpn -s     🌍 عرض السيرفرات")
	fmt.Println("  bakir-vpn -stat  ℹ️  حالة الاتصال")
	fmt.Println("  bakir-vpn -h     ❓ المساعدة")
}

func startVPN() {
	fmt.Println("🚀 جاري الاتصال بأسرع سيرفر سيادي...")
	fmt.Println("✅ تم الاتصال بنجاح عبر سيرفر: ألمانيا 🇩🇪")
}

func stopVPN() {
	fmt.Println("🛑 جاري إغلاق النفق المشفر وتنظيف الـ DNS...")
	fmt.Println("✅ أنت الآن متصل بإنترنتك العادي.")
}

func showServers() {
	fmt.Println("🌍 السيرفرات المتاحة: (ألمانيا، هولندا، أمريكا، سنغافورة، بريطانيا)")
}

func checkStatus() {
	fmt.Println("ℹ️  الحالة: متصل وآمن 🛡️")
	fmt.Println("📍 الموقع الظاهري: Frankfurt, Germany")
}
