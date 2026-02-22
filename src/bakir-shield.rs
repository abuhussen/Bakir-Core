use std::process::{Command, Stdio};
use std::env;
use std::path::Path;
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.contains(&"-h".to_string()) {
        display_help();
        return;
    }

    match args[1].as_str() {
        "-all" => handle_all_ports(&args),
        "-prt" | "-port" => handle_single_port(&args),
        "-scan" => scan_ports(),
        "-ghost" => toggle_stealth_mode(&args),
        "-guard" => start_guard(),
        _ => println!("❌ أمر غير معروف. استخدم bakir -h لعرض القائمة."),
    }
}

fn display_help() {
    println!("🛡️ حصن باكير السيادي | Bakir-Shield");
    println!("------------------------------------------");
    println!("bakir -all [on/off]        : فتح أو إغلاق شامل للمنافذ");
    println!("bakir -port [الرقم] [on/off] : التحكم بمنفذ محدد (فتح/إغلاق)");
    println!("bakir -scan                : عرض جدول المنافذ التفصيلي");
    println!("bakir -ghost [on/off]      : وضع الشبح (إخفاء الـ Ping)");
    println!("bakir -guard               : تفعيل الحارس اليقظ ومراقبته");
    println!("------------------------------------------");
}

fn handle_all_ports(args: &[String]) {
    let action = if args.contains(&"off".to_string()) { "deny" } else { "allow" };
    println!("🛡️ جاري تغيير السياسة العامة إلى: {}...", action);
    let _ = Command::new("sudo").args(&["ufw", "default", action, "incoming"]).status();
    let _ = Command::new("sudo").args(&["ufw", "--force", "enable"]).status();
    println!("✅ تم تحديث السياسة العامة للحصن.");
}

fn handle_single_port(args: &[String]) {
    if args.len() < 4 {
        println!("❌ نقص في الأوامر! مثال: bakir -port 80 on");
        return;
    }
    let port = &args[2];
    let action = if args[3] == "on" { "allow" } else { "deny" };

    println!("⚙️ جاري ضبط القيد الأمني على المنفذ {}...", port);
    let status = Command::new("sudo").args(&["ufw", action, port]).status();
    if status.is_ok() {
        println!("✅ المنفذ {} الآن في وضع: {}.", port, args[3]);
    }
}

fn scan_ports() {
    println!("📊 فحص المنافذ في حصن باكير...");
    // استخدام Stdio لجلب البيانات وعرضها فوراً
    let output = Command::new("sudo")
        .args(&["ufw", "status", "numbered"])
        .stdout(Stdio::inherit()) // هذا السطر سيجعل الجدول يظهر أمامك مباشرة
        .output()
        .expect("فشل في عرض البيانات");
}

fn toggle_stealth_mode(args: &[String]) {
    let val = if args.contains(&"on".to_string()) { "1" } else { "0" };
    let _ = Command::new("sudo").args(&["sysctl", "-w", &format!("net.ipv4.icmp_echo_ignore_all={}", val)]).status();
    println!("👻 وضع الشبح: {}", if val == "1" { "مفعّل" } else { "معطل" });
}

fn start_guard() {
    println!("📡 جاري تشغيل الحارس اليقظ لباكير...");
    // فحص rsyslog والتأكد من تفعيله
    let _ = Command::new("sudo").args(&["systemctl", "enable", "--now", "rsyslog"]).status();
    
    // إرسال رسالة تأكيد للمستخدم
    println!("🛡️ الحارس بدأ العمل ومراقبة السجلات الآن بنجاح.");
    println!("ℹ️ يمكنك متابعة السجلات عبر: sudo tail -f /var/log/auth.log");
}