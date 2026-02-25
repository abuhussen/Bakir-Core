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
	BakirCore []App `json:"bakir_core"`
}

func main() {
	fmt.Println("🛒 Bakir Store | متجر باكير السيادي v3.0")
	fmt.Println("--------------------------------------------------")

	content, err := ioutil.ReadFile("/home/bakir/Bakir-Project/remote-repo/apps.json")
	if err != nil {
		fmt.Println("❌ خطأ: لم يتم العثور على قاعدة بيانات المتجر.")
		return
	}

	var repo Repo
	json.Unmarshal(content, &repo)

	fmt.Println("📦 [Bakir Core Tools - الأدوات السيادية]")
	for i, app := range repo.BakirCore {
		fmt.Printf("%d. %-20s | %s\n", i+1, app.Name, app.Desc)
	}
	fmt.Println("\n✅ المتجر عاد للعمل بنجاح يا سيادة المستشار.")
}
