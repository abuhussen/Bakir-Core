package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"os"
	"os/exec"
)

type App struct {
	Name string `json:"name"`
	Desc string `json:"desc"`
}

type FullRepo struct {
	ExternalApps []App `json:"external_apps_list"`
}

func main() {
	if os.Geteuid() != 0 {
		fmt.Println("❌ خطأ: يرجى التشغيل بصلاحيات sudo (السيادة تتطلب السلطة!)")
		return
	}

	fmt.Println("📦 Bakir-App-Manager v2.0 | مثبت البرمجيات الخارجية")
	fmt.Println("--------------------------------------------------")

	// قراءة ملف البيانات الموسع
	data, err := ioutil.ReadFile("/home/bakir/Bakir-Project/remote-repo/apps.json")
	if err != nil {
		fmt.Println("❌ خطأ: قاعدة بيانات البرامج مفقودة.")
		return
	}

	var repo FullRepo
	json.Unmarshal(data, &repo)

	// عرض القائمة الموسعة (11 برنامج أو أكثر)
	for i, app := range repo.ExternalApps {
		fmt.Printf("%d. تثبيت %-15s | %s\n", i+1, app.Name, app.Desc)
	}

	fmt.Println("--------------------------------------------------")
	fmt.Print("اختر رقم البرنامج لتثبيته (أو 0 للخروج): ")

	var choice int
	fmt.Scanln(&choice)

	if choice > 0 && choice <= len(repo.ExternalApps) {
		target := repo.ExternalApps[choice-1]
		installLogic(target.Name)
	} else {
		fmt.Println("👋 خروج...")
	}
}

func installLogic(name string) {
	fmt.Printf("🚀 جاري تحضير بيئة التثبيت لـ %s...\n", name)
	
	// خريطة أوامر التثبيت
	commands := map[string]string{
		"Google Chrome": "wget https://dl.google.com/linux/direct/google-chrome-stable_current_amd64.deb && apt install ./google-chrome-stable_current_amd64.deb -y",
		"VS Code":       "wget -O code.deb https://update.code.visualstudio.com/latest/linux-deb-x64/stable && apt install ./code.deb -y",
		"Zoom":          "wget https://zoom.us/client/latest/zoom_amd64.deb && apt install ./zoom_amd64.deb -y",
		"Telegram":      "apt install telegram-desktop -y",
		"Discord":       "wget -O discord.deb \"https://discord.com/api/download?platform=linux&format=deb\" && apt install ./discord.deb -y",
		"Spotify":       "curl -sS https://download.spotify.com/debian/pubkey_C85661D9.gpg | gpg --dearmor | sudo tee /etc/apt/trusted.gpg.d/spotify.gpg && echo \"deb http://repository.spotify.com stable non-free\" | sudo tee /etc/apt/sources.list.d/spotify.list && apt update && apt install spotify-client -y",
		"GIMP":          "apt install gimp -y",
		"Inkscape":      "apt install inkscape -y",
		"Audacity":      "apt install audacity -y",
		"AnyDesk":       "wget -qO - https://keys.anydesk.com/repos/DEB-GPG-KEY | apt-key add - && echo \"deb http://deb.anydesk.com/ all main\" > /etc/apt/sources.list.d/anydesk-stable.list && apt update && apt install anydesk -y",
		"Docker":        "apt install docker.io -y && systemctl enable --now docker",
	}

	if cmdStr, ok := commands[name]; ok {
		cmd := exec.Command("sh", "-c", cmdStr)
		cmd.Stdout = os.Stdout
		cmd.Stderr = os.Stderr
		cmd.Run()
		fmt.Printf("✅ تم تثبيت %s بنجاح في قلب نظام باكير!\n", name)
	} else {
		fmt.Println("❌ عذراً، أمر التثبيت لهذا البرنامج قيد التجهيز.")
	}
}
