package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
)

type App struct {
	Name string `json:"name"`
	Desc string `json:"desc"`
}

type Repo struct {
	BakirCore     []App `json:"bakir_core"`
	GlobalTools   []App `json:"global_tools"`
	ThemesProject []App `json:"themes_project"`
}

func main() {
	fmt.Println("🛒 Bakir Store | متجر باكير السيادي v3.1")
	fmt.Println("--------------------------------------------------")

	content, err := ioutil.ReadFile("/home/bakir/Bakir-Project/remote-repo/apps.json")
	if err != nil {
		fmt.Println("❌ خطأ: لم يتم العثور على قاعدة بيانات المتجر.")
		return
	}

	var repo Repo
	json.Unmarshal(content, &repo)

	fmt.Println("📦 [القسم الأول - Bakir Core Tools]")
	for i, app := range repo.BakirCore {
		fmt.Printf("%d. %-20s | %s\n", i+1, app.Name, app.Desc)
	}

	fmt.Println("\n🌍 [القسم الثاني - Global Tools]")
	for i, app := range repo.GlobalTools {
		fmt.Printf("%d. %-20s | %s\n", i+1, app.Name, app.Desc)
	}

	for _, app := range repo.ThemesProject {
		fmt.Printf("* %-21s | %s\n", app.Name, app.Desc)
	}
	
	fmt.Println("--------------------------------------------------")
	fmt.Println("✅ المتجر عاد بكامل قواه يا سيادة المستشار!")
}
