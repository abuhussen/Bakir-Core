package main

import (
	"fmt"
	"os"
	"os/exec"
)

func main() {
	if os.Geteuid() != 0 {
		fmt.Println("❌ خطأ: يرجى التشغيل بصلاحيات sudo")
		return
	}

	fmt.Println("📦 Bakir-App-Manager | مثبت البرمجيات الخارجية")
	fmt.Println("--------------------------------------------------")
	fmt.Println("1. تثبيت Google Chrome")
	fmt.Println("2. تثبيت Visual Studio Code")
	fmt.Println("3. تثبيت Zoom")
	fmt.Println("--------------------------------------------------")
	fmt.Print("اختر رقم البرنامج لتثبيته: ")

	var choice int
	fmt.Scanln(&choice)

	switch choice {
	case 1:
		installApp("Google Chrome", "wget https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb && apt install ./google-chrome-stable_current_amd64.deb -y")
	case 2:
		installApp("VS Code", "wget -O code.deb https://update.code.visualstudio.com/latest/linux-deb-x64/stable && apt install ./code.deb -y")
	case 3:
		installApp("Zoom", "wget https://zoom.us/client/latest/zoom_amd64.deb && apt install ./zoom_amd64.deb -y")
	default:
		fmt.Println("❌ اختيار غير صحيح")
	}
}

func installApp(name string, command string) {
	fmt.Printf("🚀 جاري تحميل وتثبيت %s...\n", name)
	cmd := exec.Command("sh", "-c", command)
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr
	cmd.Run()
	fmt.Printf("✅ تم تثبيت %s بنجاح!\n", name)
}
