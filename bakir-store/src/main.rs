use serde::{Deserialize, Serialize};
use colored::*;
use std::process::Command;
use std::env;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct App {
    name: String,
    desc: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Repo {
    bakir_core: Vec<App>,
    global_tools: Vec<App>,
}

fn send_notification(title: &str, msg: &str, urgency: &str) {
    let _ = Command::new("notify-send")
        .args(&[title, msg, "-i", "package-x-generic", "-u", urgency, "-t", "4000"])
        .status();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // مسار ملف البيانات (سنعتمد المسار الذي تستخدمه حالياً)
    let json_path = "/home/bakir/Bakir-Project/apps.json";
    
    let content = fs::read_to_string(json_path).unwrap_or_else(|_| {
        println!("{}", "❌ خطأ: تعذر العثور على ملف apps.json في المسار المحدد.".red());
        String::from("{}")
    });

    let repo: Repo = serde_json::from_str(&content).unwrap_or(Repo {
        bakir_core: vec![],
        global_tools: vec![],
    });

    if args.len() > 2 && args[1] == "-i" {
        install_app(&args[2]);
        return;
    }

    display_menu(repo);
}

fn display_menu(repo: Repo) {
    println!("{}", "==================================================".cyan());
    println!("{}", "      📦 Bakir Store | متجر باكير السيادي v7.0     ".bold().bright_cyan());
    println!("{}", "==================================================".cyan());

    println!("\n{}", "📂 [ القسم الأول - Bakir Core Tools ]".yellow().bold());
    for app in repo.bakir_core {
        println!("  • {:<20} | {}", app.name.green().bold(), app.desc.white());
    }

    println!("\n{}", "🌍 [ القسم الثاني - Global Tools ]".blue().bold());
    for app in repo.global_tools {
        println!("  • {:<20} | {}", app.name.green().bold(), app.desc.white());
    }

    println!("\n{}", "--------------------------------------------------".cyan());
    println!("{} {}", "💡 للاستخدام:".yellow(), "bakir-store -i [اسم_الأداة]".white().italic());
    println!("{}", "==================================================".cyan());
}

fn install_app(name: &str) {
    // التحقق من صلاحيات sudo
    let output = Command::new("id").arg("-u").output().expect("فشل التحقق من الصلاحيات");
    let uid = String::from_utf8_lossy(&output.stdout).trim().parse::<u32>().unwrap();

    if uid != 0 {
        println!("{}", "❌ خطأ سيادي: التثبيت يتطلب صلاحيات sudo (استخدم sudo قبل الأمر)".red().bold());
        return;
    }

    println!("🚚 جاري معالجة تثبيت {} ...", name.cyan().bold());
    send_notification("📦 متجر باكير", &format!("بدء تثبيت أداة: {}", name), "normal");

    // هنا نضع منطق التثبيت (حالياً سنقوم بمحاكاة التثبيت أو البحث عن الملف)
    // في النسخة النهائية سنربطه بـ GitHub أو مجلد الـ Core
    let repo_path = "/home/bakir/Bakir-Project/";
    let source = format!("{}{}", repo_path, name);
    let target = format!("/usr/bin/{}", name);

    let status = Command::new("cp")
        .args(&[&source, &target])
        .status();

    match status {
        Ok(s) if s.success() => {
            let _ = Command::new("chmod").args(&["+x", &target]).status();
            println!("✅ {} {}", name.green().bold(), "تم تثبيته بنجاح!".white());
            send_notification("✅ اكتمل التثبيت", &format!("الأداة {} جاهزة للاستخدام الآن", name), "normal");
        }
        _ => {
            println!("⚠️ {} {}", "تنبيه: لم يتم العثور على ملف محلي لـ".yellow(), name.red());
            println!("{}", "جاري محاولة التثبيت عبر مدير حزم النظام (apt)...".white());
            
            let apt_status = Command::new("apt").args(&["install", "-y", name]).status();
            if apt_status.is_ok() && apt_status.unwrap().success() {
                send_notification("✅ اكتمل التثبيت", &format!("تم تثبيت {} عبر مستودعات النظام", name), "normal");
            } else {
                send_notification("❌ فشل التثبيت", &format!("تعذر العثور على {}", name), "critical");
            }
        }
    }
}
