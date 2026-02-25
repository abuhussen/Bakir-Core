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

type Repo struct {
	BakirCore   []App `json:"bakir_core"`
	GlobalTools []App `json:"global_tools"`
}

func main() {
	repoPath := "/home/bakir/Bakir-Project/remote-repo/"
	data, err := ioutil.ReadFile(repoPath + "apps.json")
	if err != nil {
		fmt.Println("❌ خطأ: تعذر الوصول لقاعدة بيانات المتجر.")
		return
	}

	var repo Repo
	json.Unmarshal(data, &repo)

	if len(os.Args) > 2 && os.Args[1] == "-i" {
		appName := os.Args[2]
		installApp(appName, repoPath)
		return
	}

	fmt.Println("--------------------------------------------------")
	fmt.Println("📦 Bakir Store | متجر باكير السيادي v3.5")
	fmt.Println("--------------------------------------------------")

	fmt.Println("\n📂 [القسم الأول - Bakir Core Tools]")
	for i, app := range repo.BakirCore {
		fmt.Printf("%d. %-20s | %s\n", i+1, app.Name, app.Desc)
	}

	fmt.Println("\n🌍 [القسم الثاني - Global Tools]")
	for i, app := range repo.GlobalTools {
		fmt.Printf("%d. %-20s | %s\n", i+1, app.Name, app.Desc)
	}
	fmt.Println("\n💡 للاستخدام: bakir-store -i [اسم_الأداة]")
	fmt.Println("--------------------------------------------------")
}

func installApp(name string, repoPath string) {
	if os.Geteuid() != 0 {
		fmt.Println("❌ خطأ سيادي: التثبيت يتطلب صلاحيات sudo")
		return
	}
	sourceFile := repoPath + name
	targetFile := "/usr/bin/" + name
	fmt.Printf("🚚 جاري سحب وتثبيت %s من المستودع السيادي...\n", name)
	cmd := exec.Command("sh", "-c", fmt.Sprintf("cp %s %s && chmod +x %s", sourceFile, targetFile, targetFile))
	err := cmd.Run()
	if err != nil {
		fmt.Printf("❌ فشل التثبيت: تأكد من وجود ملف %s في المستودع.\n", name)
	} else {
		fmt.Printf("✅ تم تثبيت %s بنجاح! يمكنك استخدامه الآن.\n", name)
	}
}
