use std::process::Command;
use std::env;

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
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("   🏪 متجر باكير الذكي | Bakir Store v3.0   ");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("الاستخدام: bakir-store -i [اسم_البرنامج]");
    println!("");
    println!("🛡️  [الأدوات السيادية - Bakir Core]");
    println!("   • bakir-shield          : الجدار الناري الذكي");
    println!("   • bakir-get             : محرك التحميل الشامل");
    println!("   • bakir-opt             : منظف ومسرع النظام");
    println!("   • bakir-terminal-theme  : مغير سمات الطرفية");
    println!("");
    println!("🌐  [الأدوات العالمية - Global Tools]");
    println!("   • timeshift             : نظام لقطات الاستعادة");
    println!("   • vlc                   : مشغل الوسائط الشامل");
    println!("   • stacer                : مراقب النظام الرسومي");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}

fn install_logic(pkg: &str) {
    match pkg {
        "bakir-shield" | "bakir-get" | "bakir-opt" | "bakir-terminal-theme" => {
            println!("🚀 جاري جلب الأداة السيادية [{}] من المستودع...", pkg);
            install_sovereign(pkg);
        },
        "timeshift" | "vlc" | "stacer" => {
            println!("🌐 جاري جلب [{}] من مستودعات Debian الرسمية...", pkg);
            install_global(pkg);
        },
        _ => println!("❌ الخطأ: البرنامج [{}] غير مدرج في سجلاتنا.", pkg),
    }
}

fn install_sovereign(name: &str) {
    // تصحيح الرابط: السكربتات والبرامج موجودة في remote-repo داخل المستودع
    let url = format!("https://raw.githubusercontent.com/abuhussen/Bakir-Core/main/remote-repo/{}", name);
    let dest = format!("/usr/bin/{}", name);

    let status = Command::new("sudo")
        .args(&["wget", "-q", "--show-progress", &url, "-O", &dest])
        .status()
        .expect("فشل في الاتصال بالمستودع");

    if status.success() {
        let _ = Command::new("sudo").args(&["chmod", "+x", &dest]).status();
        println!("✅ تم التثبيت بنجاح! يمكنك الآن كتابة [{}] في الطرفية.", name);
    } else {
        println!("❌ فشل التحميل. تأكد من وجود الملف في المستودع بهذا الاسم: {}", name);
    }
}

fn install_global(name: &str) {
    let status = Command::new("sudo")
        .args(&["apt", "install", "-y", name])
        .status()
        .expect("فشل في تنفيذ الأمر");

    if status.success() {
        println!("✅ تم تثبيت البرنامج العالمي [{}] بنجاح.", name);
    }
}