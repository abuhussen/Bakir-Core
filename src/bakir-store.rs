use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 { display_help(); return; }

    let action = &args[1];
    let package = &args[2];

    match action.as_str() {
        "-i" | "--install" => install_logic(package),
        _ => display_help(),
    }
}

fn display_help() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("   🏪 متجر باكير الذكي | Bakir Store v3.2   ");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("🛡️  [الأدوات السيادية]");
    println!("   • bakir-shield          • bakir-get");
    println!("   • bakir-opt             • bakir-terminal-theme");
    println!("");
    println!("🌐  [الأدوات العالمية]");
    println!("   • timeshift    • vlc    • stacer");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

fn install_logic(pkg: &str) {
    match pkg {
        "bakir-shield" => install_sovereign("bakir", "bakir-shield"), // يسحب 'bakir' ويسميه 'bakir-shield'
        "bakir-get" | "bakir-opt" | "bakir-terminal-theme" => install_sovereign(pkg, pkg),
        "timeshift" | "vlc" | "stacer" => install_global(pkg),
        _ => println!("❌ البرنامج [{}] غير مدرج.", pkg),
    }
}

fn install_sovereign(repo_name: &str, local_name: &str) {
    let url = format!("https://raw.githubusercontent.com/abuhussen/Bakir-Core/main/remote-repo/{}", repo_name);
    let dest = format!("/usr/bin/{}", local_name);

    println!("🚀 جاري سحب [{}] من المستودع السيادي...", repo_name);
    let status = Command::new("sudo")
        .args(&["wget", "-q", "--show-progress", &url, "-O", &dest])
        .status()
        .expect("فشل الاتصال");

    if status.success() {
        let _ = Command::new("sudo").args(&["chmod", "+x", &dest]).status();
        println!("✅ تم تثبيت [{}] بنجاح في نظامك.", local_name);
    } else {
        println!("❌ فشل! تأكد من وجود ملف باسم [{}] في GitHub/remote-repo", repo_name);
    }
}

fn install_global(name: &str) {
    println!("🌐 جاري جلب [{}] من مستودعات Debian...", name);
    let _ = Command::new("sudo").args(&["apt", "install", "-y", name]).status();
}