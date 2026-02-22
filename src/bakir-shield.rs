use std::process::Command;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "-h" {
        display_help();
        return;
    }

    match args[1].as_str() {
        "-all" => handle_all_ports(&args),
        "-prt" => handle_single_port(&args),
        "-scan" => scan_ports(),
        "-ghost" => toggle_stealth_mode(&args),
        "-guard" => start_guard(),
        _ => println!("❌ أمر غير معروف. استخدم bakir -h لعرض القائمة."),
    }
}

fn display_help() {
    println!("🛡️ Bakir-Shield | حصن باقر السيادي");
    println!("------------------------------------------");
    println!("bakir -all port off  : إغلاق كل المنافذ الخطرة");
    println!("bakir -all port on   : فتح كل المنافذ");
    println!("bakir -prt [رقم] on  : فتح منفذ محدد");
    println!("bakir -prt [رقم] off : إغلاق منفذ محدد");
    println!("bakir -scan          : عرض المنافذ النشطة");
    println!("bakir -ghost on/off  : الوضع الخفي (تجاهل الـ Ping)");
    println!("bakir -guard         : تفعيل الحارس اليقظ في الخلفية");
    println!("------------------------------------------");
}

fn send_plasma_notify(title: &str, msg: &str) {
    let _ = Command::new("notify-send")
        .args(&[title, msg, "-i", "security-high", "-a", "Bakir Shield"])
        .status();
}

fn handle_all_ports(args: &[String]) {
    if args.contains(&"off".to_string()) {
        let _ = Command::new("sudo").args(&["ufw", "--force", "enable"]).status();
        let _ = Command::new("sudo").args(&["ufw", "default", "deny", "incoming"]).status();
        send_plasma_notify("🛡️ الجدار الناري", "تم تفعيل وضع الحماية القصوى.");
    } else {
        let _ = Command::new("sudo").args(&["ufw", "default", "allow", "incoming"]).status();
        send_plasma_notify("🛡️ الجدار الناري", "تم فتح المنافذ الافتراضية.");
    }
}

fn handle_single_port(args: &[String]) {
    if args.len() < 4 { return; }
    let port = &args[2];
    let action = &args[3];
    let _ = Command::new("sudo").args(&["ufw", action, port]).status();
    send_plasma_notify("🛡️ تحديث المنفذ", &format!("تم {} المنفذ {}.", action, port));
}

fn scan_ports() {
    let output = Command::new("sudo").args(&["ufw", "status", "numbered"]).output().expect("فشل تنفيذ الأمر");
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn toggle_stealth_mode(args: &[String]) {
    if args.contains(&"on".to_string()) {
        let _ = Command::new("sudo").args(&["sysctl", "-w", "net.ipv4.icmp_echo_ignore_all=1"]).status();
        send_plasma_notify("👻 الوضع الخفي", "النظام الآن غير مرئي.");
    } else {
        let _ = Command::new("sudo").args(&["sysctl", "-w", "net.ipv4.icmp_echo_ignore_all=0"]).status();
        send_plasma_notify("🌐 الوضع الخفي", "النظام الآن مرئي.");
    }
}

fn start_guard() {
    println!("📡 جاري فحص بيئة الحماية...");
    
    // تأكد من وجود ufw
    if !Path::new("/usr/sbin/ufw").exists() {
        println!("❌ خطأ: أداة ufw غير مثبتة. يرجى تثبيتها أولاً.");
        return;
    }

    // تأكد من وجود ملف السجلات
    if !Path::new("/var/log/auth.log").exists() {
        println!("⚠️ تنبيه: ملف السجلات غير موجود. جاري تفعيل خدمة rsyslog...");
        let _ = Command::new("sudo").args(&["apt", "install", "-y", "rsyslog"]).status();
        let _ = Command::new("sudo").args(&["systemctl", "start", "rsyslog"]).status();
    }

    println!("✅ الحارس اليقظ بدأ العمل الآن في الخلفية...");
    send_plasma_notify("📡 حارس باقر", "بدأ الحارس بمراقبة محاولات الاختراق الآن.");
}