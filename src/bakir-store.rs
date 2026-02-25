use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1] == "-h" || args[1] == "--help" {
        display_help();
        return;
    }
    if args.len() < 3 { return; }
    let action = &args[1];
    let package = &args[2];
    match action.as_str() {
        "-i" | "--install" => install_logic(package),
        _ => display_help(),
    }
}

fn display_help() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("   🏪 متجر باكير الذكي | Bakir Store v4.7   ");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🛡️  [الأدوات السيادية المتاحة]:");
    
    // جلب القائمة الديناميكية من السحاب
    let output = Command::new("curl")
        .args(&["-s", "https://raw.githubusercontent.com/abuhussen/Bakir-Core/main/remote-repo/apps.json"])
        .output();
    
    if let Ok(out) = output {
        let s = String::from_utf8_lossy(&out.stdout);
        for line in s.lines() {
            if line.contains("\"name\"") {
                let name = line.split('"').nth(3).unwrap_or("");
                println!("   • {}", name);
            }
        }
    }

    // إضافة الأدوات التي نثبت وجودها يدوياً لضمان الظهور الدائم
    println!("   • bakir-opt             (أداة التحسين ⚡)");
    println!("   • bakir-terminal-theme  (هوية النظام 🎨)");

    println!("\n🌐  [الأدوات العالمية]:");
    println!("   • vlc          • timeshift    • stacer");
    println!("   • firefox      • vscode       • gimp");
    
    println!("\n💡 للتثبيت: bakir-store -i [الاسم]");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

fn install_logic(pkg: &str) {
    println!("🔍 فحص المستودعات لـ [{}]...", pkg);
    let url = format!("https://raw.githubusercontent.com/abuhussen/Bakir-Core/main/remote-repo/{}", pkg);
    let check = Command::new("curl").args(&["-s", "--head", "--fail", &url]).status();

    if check.is_ok() && check.unwrap().success() {
        install_sovereign(pkg, &url);
    } else {
        install_global(pkg);
    }
}

fn install_sovereign(pkg: &str, url: &str) {
    let dest = format!("/usr/bin/{}", pkg);
    println!("🚀 جلب أداة سيادية: [{}]...", pkg);
    let status = Command::new("sudo").args(&["wget", "-q", "--show-progress", url, "-O", &dest]).status();
    if status.is_ok() && status.unwrap().success() {
        let _ = Command::new("sudo").args(&["chmod", "+x", &dest]).status();
        println!("✅ تم التثبيت بنجاح.");
    }
}

fn install_global(pkg: &str) {
    println!("🌐 جلب أداة عالمية: [{}]...", pkg);
    let _ = Command::new("sudo").args(&["apt", "install", "-y", pkg]).status();
}
