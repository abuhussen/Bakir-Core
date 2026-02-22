use std::process::Command;
use std::env;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        display_help();
        return;
    }

    let action = &args[1];
    let package = &args[2];

    match action.as_str() {
        "-i" | "--install" => install_logic(package),
        _ => display_help(),
    }
}

fn display_help() {
    println!("🏪 متجر باكير الذكي | Bakir Store");
    println!("----------------------------------");
    println!("الاستخدام: bakir-store -i [اسم_البرنامج]");
    println!("\n📦 البرامج المتوفرة:");
    println!("  [أدوات سيادية]: bakir-shield, bakir-get, bakir-opt");
    println!("  [أدوات عالمية]: timeshift, vlc, stacer");
}

fn install_logic(pkg: &str) {
    match pkg {
        // فئة الأدوات السيادية (تحميل من GitHub)
        "bakir-shield" | "bakir-get" | "bakir-opt" => {
            println!("🛡️ جاري جلب الأداة السيادية [{}] من مستودع باكير...", pkg);
            install_sovereign(pkg);
        },
        // فئة الأدوات العالمية (جلب من مستودعات Debian)
        "timeshift" | "vlc" | "stacer" => {
            println!("🌐 جاري جلب [{}] من مستودعات Debian الرسمية...", pkg);
            install_global(pkg);
        },
        _ => println!("❌ البرنامج [{}] غير مدرج في قائمة المتجر حالياً.", pkg),
    }
}

fn install_sovereign(name: &str) {
    let url = format!("https://raw.githubusercontent.com/abuhussen/Bakir-Core/main/remote-repo/{}", name);
    let dest = format!("/usr/bin/{}", name);

    let status = Command::new("sudo")
        .args(&["wget", "-q", "--show-progress", &url, "-O", &dest])
        .status()
        .expect("فشل في تحميل الأداة");

    if status.success() {
        Command::new("sudo").args(&["chmod", "+x", &dest]).status().unwrap();
        println!("✅ تم تنصيب الأداة السيادية [{}] بنجاح في /usr/bin", name);
    }
}

fn install_global(name: &str) {
    // تحديث المستودعات قبل الجلب لضمان أحدث نسخة
    let _ = Command::new("sudo").args(&["apt", "update", "-y"]).status();
    
    let status = Command::new("sudo")
        .args(&["apt", "install", "-y", name])
        .status()
        .expect("فشل في الاتصال بمستودعات Debian");

    if status.success() {
        println!("✅ تم تنصيب البرنامج العالمي [{}] بنجاح عبر نظام الحزم.", name);
    }
}