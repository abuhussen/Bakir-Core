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
    aesthetics: Option<Vec<App>>
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
    println!("\n{}\n{}\n{}", "=".repeat(60).cyan(), "      🏛️ متجر باكير السيادي | Bakir-Store v19.0".bold(), "=".repeat(60).cyan());
    println!("\n{}", "🛡️ [ الأدوات الأساسية ]".yellow().bold());
    for app in root.bakir_core { println!("  • {:<20} | {}", app.name.green().bold(), app.desc.white()); }
    if let Some(aes) = root.aesthetics {
        println!("\n{}", "🎨 [ الجماليات السيادية ]".magenta().bold());
        for app in aes { println!("  • {:<20} | {}", app.name.magenta().bold(), app.desc.white()); }
    }
    println!("\n{}", "🌐 [ البرامج العالمية ]".blue().bold());
    for app in root.global_tools { println!("  • {:<20} | {}", app.name.cyan().bold(), app.desc.white()); }
    println!("\n{}", "=".repeat(60).cyan());
}

fn install_tool(name: &str) {
    // المسار الجديد الصارم والمجرب بناءً على صورك
    let project_path = format!("/home/bakir/Bakir-Project/{}", name);
    
    if std::path::Path::new(&project_path).exists() {
        println!("✅ تم العثور على المسار السيادي: {}", project_path.green());
        let is_rust = std::path::Path::new(&format!("{}/Cargo.toml", project_path)).exists();
        let bash_script = format!("{}/{}.sh", project_path, name);

        if is_rust {
            println!("🚀 بناء أداة Rust...");
            let _ = Command::new("cargo").arg("build").arg("--release").current_dir(&project_path).status();
            let _ = Command::new("sudo").arg("cp").arg(format!("{}/target/release/{}", project_path, name)).arg("/usr/local/bin/").status();
        } else if std::path::Path::new(&bash_script).exists() {
            println!("🎨 تفعيل الثيم السيادي فوراً...");
            let _ = Command::new("chmod").arg("+x").arg(&bash_script).status();
            let _ = Command::new("bash").arg(&bash_script).status();
        } else {
            println!("⚠️ المجلد موجود ولكن ملف .sh غير موجود باسم: {}.sh", name);
        }
    } else {
        println!("🌐 تحميل عالمي عبر apt: {}", name.cyan());
        let _ = Command::new("sudo").arg("apt").arg("install").arg("-y").arg(name).status();
    }
}
