use serde::{Deserialize, Serialize};
use std::fs;
use std::env;
use colored::*;
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
struct App { name: String, desc: String }
#[derive(Serialize, Deserialize, Debug)]
struct Root { 
    bakir_core: Vec<App>, 
    global_tools: Vec<App>,
    aesthetics: Option<Vec<App>> // القسم الجديد للجماليات
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 && args[1] == "-i" {
        install_tool(&args[2]);
        return;
    }
    display_store();
}

fn display_store() {
    let path = "/home/bakir/Bakir-Project/apps.json";
    let data = fs::read_to_string(path).expect("❌ ملف apps.json مفقود");
    let root: Root = serde_json::from_str(&data).expect("❌ خطأ في التنسيق");
    
    println!("\n{}\n{}\n{}", "=".repeat(60).cyan(), "      🏛️ متجر باكير السيادي | Bakir-Store v18.0".bold(), "=".repeat(60).cyan());
    
    println!("\n{}", "🛡️ [ الأدوات الأساسية - Bakir Core ]".yellow().bold());
    for app in root.bakir_core { println!("  • {:<20} | {}", app.name.green().bold(), app.desc.white()); }

    if let Some(aes) = root.aesthetics {
        println!("\n{}", "🎨 [ الجماليات السيادية - Royal Aesthetics ]".magenta().bold());
        for app in aes { println!("  • {:<20} | {}", app.name.magenta().bold(), app.desc.white()); }
    }

    println!("\n{}", "🌐 [ البرامج العالمية - Global Apps ]".blue().bold());
    for app in root.global_tools { println!("  • {:<20} | {}", app.name.cyan().bold(), app.desc.white()); }
    println!("\n{}", "=".repeat(60).cyan());
}

fn install_tool(name: &str) {
    let home = env::var("HOME").unwrap();
    let project_path = format!("{}/Bakir-Project/{}", home, name);
    
    if std::path::Path::new(&project_path).exists() {
        // التحقق: هل هو برنامج Rust أم Bash؟
        let is_rust = std::path::Path::new(&format!("{}/Cargo.toml", project_path)).exists();
        let bash_script = format!("{}/{}.sh", project_path, name);
        let is_bash = std::path::Path::new(&bash_script).exists();

        if is_rust {
            println!("🚀 بناء أداة Rust سيادية: {}", name.green().bold());
            let _ = Command::new("cargo").arg("build").arg("--release").current_dir(&project_path).status();
            let _ = Command::new("sudo").arg("cp").arg(format!("{}/target/release/{}", project_path, name)).arg("/usr/local/bin/").status();
            println!("✅ تم تثبيت أداة Rust: {}", name.bright_green());
        } else if is_bash {
            println!("🎨 تفعيل الثيم/السكربت السيادي: {}", name.magenta().bold());
            // إعطاء صلاحية التنفيذ وتشغيله فوراً
            let _ = Command::new("chmod").arg("+x").arg(&bash_script).status();
            let _ = Command::new("bash").arg(&bash_script).status();
            println!("✨ تم تفعيل {} بنجاح!", name.bright_magenta());
        }
    } else {
        println!("🌐 تحميل عالمي: {}", name.cyan());
        let _ = Command::new("sudo").arg("apt").arg("install").arg("-y").arg(name).status();
    }
}
