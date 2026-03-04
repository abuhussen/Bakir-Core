use serde::{Deserialize, Serialize};
use std::fs;
use colored::*;

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
    let path = "/home/bakir/Bakir-Project/apps.json";
    let data = fs::read_to_string(path).expect("❌ خطأ: ملف apps.json مفقود");
    let root: Root = serde_json::from_str(&data).expect("❌ خطأ في التنسيق");

    println!("{}", "\n==================================================".cyan());
    println!("{}", "   🏛️ متجر باكير السيادي | Bakir-Store v12.0".bold().bright_white());
    println!("{}", "==================================================".cyan());

    println!("\n{}", "🛡️ [ الأدوات الأساسية - Core Tools ]".yellow().bold());
    for app in root.bakir_core {
        println!("  {} {:<20} | {}", "•".green(), app.name.green().bold(), app.desc.white());
    }

    println!("\n{}", "🌐 [ البرامج العالمية - Global Apps ]".yellow().bold());
    for app in root.global_tools {
        println!("  {} {:<20} | {}", "•".blue(), app.name.blue().bold(), app.desc.white());
    }

    println!("{}", "\n==================================================".cyan());
    
    // سطر التنبيه المصحح حسب توجيهات المستشار
    println!("{}", "💡 تعليمات التشغيل والتثبيت:".yellow().bold());
    println!("  {} {} {}", "لتحميل أداة اكتب:".white(), "bakir-store -i".bright_magenta().bold(), "[اسم الأداة]".white());
    println!("  {} {} {}", "لتشغيل أداة سيادية:".white(), "اكتب اسمها مباشرة".green().bold(), "في التيرمنال".white());
    
    println!("{}", "==================================================\n".cyan());
}
