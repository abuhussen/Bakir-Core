use serde::{Deserialize, Serialize};
use std::fs;
use std::env;
use colored::*;
use std::process::Command;

#[derive(Serialize, Deserialize, Debug)]
struct App {
    name: String,
    desc: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Root {
    bakir_core: Vec<App>,
    global_tools: Vec<App>,
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
    println!("\n{}\n{}\n{}", "=".repeat(50).cyan(), "    🏛️ متجر باكير السيادي | Bakir-Store v15.0".bold(), "=".repeat(50).cyan());
    println!("\n{}", "🛡️ [ الأدوات الأساسية ]".yellow().bold());
    for app in root.bakir_core { println!("  • {:<20} | {}", app.name.green().bold(), app.desc.white()); }
    println!("\n{}", "🌐 [ البرامج العالمية ]".yellow().bold());
    for app in root.global_tools { println!("  • {:<20} | {}", app.name.blue().bold(), app.desc.white()); }
}

fn install_tool(name: &str) {
    let home = env::var("HOME").unwrap();
    let project_path = format!("{}/Bakir-Project/{}", home, name);
    if std::path::Path::new(&project_path).exists() {
        println!("🏗️ بناء أداة سيادية من: {}", project_path);
        let _ = Command::new("cargo").arg("build").arg("--release").current_dir(&project_path).status();
        let _ = Command::new("sudo").arg("cp").arg(format!("{}/target/release/{}", project_path, name)).arg("/usr/local/bin/").status();
        println!("✅ تم تثبيت الأداة السيادية: {}", name.green().bold());
    } else {
        println!("🌐 تحميل برنامج عالمي: {}", name.cyan());
        let _ = Command::new("sudo").arg("apt").arg("install").arg("-y").arg(name).status();
    }
}
