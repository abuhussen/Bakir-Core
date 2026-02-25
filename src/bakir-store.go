package main

import (
	"encoding/json"
	"fmt"
	"io/ioutil"
	"os"
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
	fmt.Println("--------------------------------------------------")
	fmt.Println("📦 Bakir Store | متجر باكير السيادي v3.1")
	fmt.Println("--------------------------------------------------")

	// قراءة قاعدة البيانات
	data, err := ioutil.ReadFile("/home/bakir/Bakir-Project/remote-repo/apps.json")
	if err != nil {
		fmt.Println("❌ خطأ: تعذر الوصول لقاعدة بيانات المتجر.")
		return
	}

	var repo Repo
	json.Unmarshal(data, &repo)

	// القسم الأول: الأدوات الأساسية
	fmt.Println("\n📂 [القسم الأول - Bakir Core Tools]")
	for i, app := range repo.BakirCore {
		fmt.Printf("%d. %-20s | %s\n", i+1, app.Name, app.Desc)
	}

	// القسم الثاني: الأدوات العالمية
	fmt.Println("\n🌍 [القسم الثاني - Global Tools]")
	for i, app := range repo.GlobalTools {
		fmt.Printf("%d. %-20s | %s\n", i+1, app.Name, app.Desc)
	}

	fmt.Println("--------------------------------------------------")
}
