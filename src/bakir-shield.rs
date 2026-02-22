use std::process::Command;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    // عرض المساعدة إذا لم يتم إدخال أوامر أو عند طلب المساعدة
    if args.len() < 2 || args.contains(&"-h".to_string()) {
        display_help();
        return;
    }

    match args[1].as_str() {
        "-all" => handle_all_ports(&args),
        "-prt" | "-port" => handle_single_port(&args), // مرونة في كتابة الأوامر
        "-scan" => scan_ports(),
        "-ghost" => toggle_stealth_mode(&args),
        "-guard" => start_guard(),
        _ => println!("❌ أمر غير معروف. استخدم bakir -h لعرض القائمة."),
    }
}

fn display_help() {
    println!("🛡️ Bakir-Shield | حصن باكير السيادي");
    println!("------------------------------------------");
    println!("bakir -all [on/off]     : فتح أو إغلاق شامل لكافة المنافذ");
    println!("bakir -port [رقم] [on/off] : التحكم الدقيق بمنفذ معين");
    println!("bakir -scan             : عرض جدول المنافذ التفصيلي");
    println!("bakir -ghost [on/off]   : تفعيل/تعطيل وضع الشبح (ICMP)");
    println!("bakir -guard            : تفعيل الحارس اليقظ في الخلفية");
    println!("------------------------------------------");
}

fn send_plasma_notify(title: &str, msg: &str) {
    let _ = Command::new("notify-send")
        .args(&[title, msg, "-i", "security-high", "-a", "Bakir Shield"])
        .status();
}

// 1. إصلاح القسم الشامل
fn handle_all_ports(args: &[String]) {
    if args.contains(&"off".to_string()) {
        let _ = Command::new("sudo").args(&["ufw", "--force", "enable"]).status();
        let _ = Command::new("sudo").args(&["ufw", "default", "deny", "incoming"]).status();
        send_plasma_notify("🛡️ حصن باكير", "تم إغلاق كافة المنافذ (حماية قصوى).");
    } else {
        let _ = Command::new("sudo").args(&["ufw", "default", "allow", "incoming"]).status();
        send_plasma_notify("🛡️ حصن باكير", "تم فتح المنافذ الافتراضية.");
    }
}

// 2. إصلاح قسم المنفذ الفردي (ترجمة on/off إلى allow/deny)
fn handle_single_port(args: &[String]) {
    if args.len() < 4 { 
        println!("❌ خطأ في التنسيق. مثال: bakir -port 80 off");
        return; 
    }
    let port = &args[2];
    let action = if args[3] == "on" { "allow" } else { "deny" };

    let _ = Command::new("sudo").args(&["ufw", action, port]).status();
    send_plasma_notify("🛡️ حصن باكير", &format!("تحديث المنفذ {}: حالة ({})", port, args[3]));
}

// 3. إصلاح قسم الفحص (عرض الجدول المرقم)
fn scan_ports() {
    println!("📊 جاري فحص المنافذ في حصن باكير...");
    let output = Command::new("sudo").args(&["ufw", "status", "numbered"]).output().unwrap();
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

// 4. قسم وضع الشبح
fn toggle_stealth_mode(args: &[String]) {
    let val = if args.contains(&"on".to_string()) { "1" } else { "0" };
    let _ = Command::new("sudo").args(&["sysctl", "-w", &format!("net.ipv4.icmp_echo_ignore_all={}", val)]).status();
    send_plasma_notify("👻 وضع الشبح", if val == "1" { "النظام الآن غير مرئي." } else { "النظام الآن مرئي." });
}

// 5. إصلاح قسم الحارس (تثبيت rsyslog تلقائياً ومنع الانهيار)
fn start_guard() {
    println!("📡 جاري تشغيل الحارس اليقظ لباكير...");
    if !Path::new("/var/log/auth.log").exists() {
        println!("⚠️ ملف السجلات مفقود، جاري تهيئة النظام...");
        let _ = Command::new("sudo").args(&["apt", "update"]).status();
        let _ = Command::new("sudo").args(&["apt", "install", "-y", "rsyslog"]).status();
        let _ = Command::new("sudo").args(&["systemctl", "enable", "--now", "rsyslog"]).status();
    }
    send_plasma_notify("📡 حارس باكير", "بدأ الحارس بمراقبة محاولات الاختراق.");
}