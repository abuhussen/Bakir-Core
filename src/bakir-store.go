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
	BakirCore []App `json:"bakir_core"`
}

func main() {
	fmt.Println("🛒 Bakir Store | متجر باكير السيادي v3.0")
	fmt.Println("--------------------------------------------------")

	// قراءة ملف البيانات الذكي
	data, err := ioutil.ReadFile("/home/bakir/Bakir-Project/remote-repo/apps.json")
	if err != nil {
		fmt.Println("❌ خطأ: لا يمكن الوصول لقاعدة بيانات المتجر.")
		return
	}

	var repo Repo
	json.Unmarshal(data, &repo)

	fmt.Println("📦 [القسم الأول - Bakir Core Tools]")
	for i, app := range repo.BakirCore {
		fmt.Printf("%d. %-20s | %s\n", i+1, app.Name, app.Desc)
	}
	
	fmt.Println("\n✅ المتجر الآن محدث تلقائياً من السحاب.")
}
