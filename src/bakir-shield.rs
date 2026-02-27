use std::process::{Command};
use std::env;
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

const WHITELIST: &[&str] = &["127.0.0.1", "192.168"];

fn send_notification(title: &str, msg: &str, urgency: &str) {
    // إرسال إشعار للنظام لضمان الظهور الفوري
    let _ = Command::new("notify-send")
        .args(&[title, msg, "-i", "security-high", "-u", urgency])
        .status();
}

fn monitor_logs() {
    send_notification("📡 حارس باكير", "الدرع الدفاعي يراقب الآن بصمت.", "normal");
    let log_path = "/var/log/auth.log";
    let file = File::open(log_path).unwrap_or_else(|_| {
        Command::new("sudo").args(&["touch", log_path]).status().unwrap();
        File::open(log_path).expect("❌ لا يمكن فتح السجلات")
    });
    
    let mut reader = BufReader::new(file);
    let _ = reader.seek(SeekFrom::End(0));

    loop {
        let mut line = String::new();
        if let Ok(len) = reader.read_line(&mut line) {
            if len > 0 && line.contains("Failed password") {
                // منطق الحظر التلقائي هنا
            }
        }
        thread::sleep(Duration::from_millis(800));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { display_help(); return; }

    match args[1].as_str() {
        "-all" => {
            if args.len() > 2 && args[2] == "on" {
                let _ = Command::new("sudo").args(&["ufw", "disable"]).status();
                let _ = Command::new("sudo").args(&["iptables", "-F"]).status();
                println!("🚀 تم فتح الشبكة بالكامل.");
            } else {
                let _ = Command::new("sudo").args(&["ufw", "default", "deny", "incoming"]).status();
                println!("🔒 تم إغلاق كافة المنافذ الواردة.");
            }
        },
        "-port" => {
            if args.len() > 3 {
                let action = if args[3] == "on" { "allow" } else { "deny" };
                let _ = Command::new("sudo").args(&["ufw", action, &args[2]]).status();
                send_notification("🔌 تحديث المنفذ", &format!("تم {} المنفذ {}", action, args[2]), "normal");
            }
        },
        "-guard" => monitor_logs(),
        "-stop" => {
            let _ = Command::new("sudo").args(&["pkill", "-f", "bakir-shield"]).status();
            let _ = Command::new("sudo").args(&["ufw", "disable"]).status();
            send_notification("🛑 إيقاف", "تم إيقاف الدرع وفتح الشبكة.", "normal");
        },
        "-redbutton" => {
            let _ = Command::new("sudo").args(&["ufw", "deny", "22"]).status();
            let _ = Command::new("sudo").args(&["ufw", "deny", "21"]).status();
            send_notification("🚨 زر الطوارئ", "تم عزل المنافذ الحساسة فوراً.", "critical");
        },
        "-status" => {
            println!("📊 تقرير درع باكير v4.2:");
            let _ = Command::new("sudo").args(&["ufw", "status", "numbered"]).status();
        },
        "-scan" => {
            println!("🔍 فحص المنافذ المفتوحة...");
            let _ = Command::new("sudo").args(&["ss", "-tuln"]).status();
            send_notification("🔍 فحص", "اكتمل فحص المنافذ النشطة.", "normal");
        },
        _ => display_help(),
    }
}

fn display_help() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("   ⚔️ Bakir-Shield v4.2 | الإصدار الكامل ");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  • bakir-shield -all on/off      : التحكم الشامل");
    println!("  • bakir-shield -port [رقم] on/off: التحكم بمنفذ");
    println!("  • bakir-shield -guard           : الحارس الصامت");
    println!("  • bakir-shield -stop            : إيقاف كل شيء");
    println!("  • bakir-shield -redbutton       : الطوارئ الذكي");
    println!("  • bakir-shield -status          : التقرير الأمني");
    println!("  • bakir-shield -scan            : فحص الشبكة");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}
