package main

import (
	"fmt"
	"os"
	"os/exec"
	"path/filepath"
	"sort"
	"time"
)

func main() {
	fmt.Println("🛡️ Bakir-Snap v1.0 | نظام النسخ الاحتياطي الذكي")
	fmt.Println("--------------------------------------------------")

	homeDir, _ := os.UserHomeDir()
	backupDir := filepath.Join(homeDir, "Bakir-Backups")
	
	// إنشاء مجلد النسخ إذا لم يكن موجوداً
	os.MkdirAll(backupDir, os.ModePerm)

	timestamp := time.Now().Format("2006-01-02_15-04-05")
	archiveName := filepath.Join(backupDir, fmt.Sprintf("snap_%s.tar.gz", timestamp))

	// 1. عملية الضغط (الأدلة والملفات الهامة)
	fmt.Println("📦 جاري إنشاء نسخة احتياطية ذكية...")
	// قمنا بإضافة .bashrc و .zshrc ومجلد .config كاملاً
	cmd := exec.Command("tar", "-czf", archiveName, "-C", homeDir, ".bashrc", ".zshrc", ".config")
	err := cmd.Run()

	if err != nil {
		fmt.Printf("⚠️ ملاحظة: تم النسخ مع تخطي بعض الملفات غير الموجودة.\n")
	}

	// 2. إدارة المساحة (التدوير الذكي - إبقاء آخر 3 نسخ فقط)
	rotateBackups(backupDir)

	fmt.Println("--------------------------------------------------")
	fmt.Printf("✅ تم الحفظ بنجاح في: %s\n", archiveName)
	fmt.Println("🧹 تم فحص المساحة وإزالة النسخ القديمة تلقائياً.")
}

func rotateBackups(dir string) {
	files, err := filepath.Glob(filepath.Join(dir, "snap_*.tar.gz"))
	if err != nil || len(files) <= 3 {
		return
	}

	// ترتيب الملفات حسب وقت الإنشاء
	sort.Strings(files)

	// حذف الملفات الأقدم والإبقاء على آخر 3 فقط
	for i := 0; i < len(files)-3; i++ {
		os.Remove(files[i])
	}
}
