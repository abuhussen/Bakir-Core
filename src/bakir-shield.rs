use std::process::{Command};
use std::env;
use std::thread;
use std::time::Duration;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Seek, SeekFrom, Write};
use std::path::Path;

// --- القائمة البيضاء (حمايتك من الحظر الذاتي) ---
const WHITELIST: &[&str] = &["127.0.0.1", "192.168.1", "192.168.0"];

fn send_notification(title: &str, msg: &str, urgency: &str) {
    let _ = Command::new("sudo")
        .args(&["-u", "bakir", "DISPLAY=:0", "DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus", 
                "notify-send", title, msg, "-i", "security-high", "-u", urgency])
        .status();
}

fn defense_action(target_ip: &str) {
    // التحقق من القائمة البيضاء قبل أي إجراء
    for white_ip in WHITELIST {
        if target_ip.starts_with(white_ip) { return; }
    }

    send_notification("🛡️ تأمين سيادي", &format!("تم رصد محاولة اختراق من {}. تم الحظر فوراً.", target_ip), "normal");
    let _ = Command::new("sudo").args(&["iptables", "-A", "INPUT", "-s", target_ip, "-j", "DROP"]).status();
}

fn monitor_logs() {
    let log_path = "/var/log/auth.log";
    if !Path::new(log_path).exists() {
        let _ = Command::new("sudo").args(&["touch", log_path]).status();
        let _ = Command::new("sudo").args(&["chmod", "644", log_path]).status();
    }
    
    let file = File::open(log_path).expect("❌ فشل الوصول للسجلات!");
    let mut reader = BufReader::new(file);
    let _ = reader.seek(SeekFrom::End(0));

    println!("📡 الحارس الصامت نشط الآن... الموارد مستقرة.");
    send_notification("📡 حارس باكير", "بدأ الدرع الدفاعي المراقبة بصمت.", "normal");

    loop {
        let mut line = String::new();
        if let Ok(len) = reader.read_line(&mut line) {
            if len > 0 && line.contains("Failed password") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                for (i, part) in parts.iter().enumerate() {
                    if *part == "from" && i + 1 < parts.len() {
                        defense_action(parts[i+1]);
                    }
                }
            }
        }
        thread::sleep(Duration::from_millis(800)); // توفير موارد المعالج (ذكاء اصطناعي برمي)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { display_help(); return; }

    match args[1].as_str() {
        "-all" => {
            if args.contains(&"on".to_string()) {
                let _ = Command::new("sudo").args(&["ufw", "disable"]).status();
                let _ = Command::new("sudo").args(&["iptables", "-F"]).status();
                let _ = Command::new("sudo").args(&["ufw", "default", "allow", "incoming"]).status();
                let _ = Command::new("sudo").args(&["ufw", "default", "allow", "outgoing"]).status();
                let _ = Command::new("sudo").args(&["ufw", "--force", "enable"]).status();
                println!("🚀 الاتصالات مفتوحة ومؤمنة بالدرع.");
            } else {
                let _ = Command::new("sudo").args(&["ufw", "default", "deny", "incoming"]).status();
                println!("🔒 تم إغلاق المنافذ الواردة.");
            }
        },
        "-guard" => monitor_logs(),
        "-stop" => {
            let _ = Command::new("sudo").args(&["pkill", "-f", "bakir-shield"]).status();
            println!("🛑 تم إيقاف الحارس الصامت بنجاح.");
        },
        "-redbutton" => {
            // الطوارئ الذكي: حظر المنافذ الحساسة + ترك المتصفح يعمل
            let ports = ["21", "22", "23", "445"];
            for port in &ports {
                let _ = Command::new("sudo").args(&["ufw", "deny", port]).status();
            }
            send_notification("🚨 وضع الحصن", "تم إغلاق المنافذ الحساسة (تصفحك لا يزال متاحاً).", "critical");
            println!("🚨 تم تفعيل الحماية الذكية.");
        },
        "-clean" => {
            let _ = Command::new("sudo").args(&["iptables", "-F"]).status();
            println!("🧹 تم تصفير قائمة الحظر بنجاح.");
        },
        "-status" => {
            println!("📊 حالة درع باكير:");
            let _ = Command::new("sudo").args(&["iptables", "-L", "INPUT", "-v", "-n"]).status();
        },
        _ => display_help(),
    }
}

fn display_help() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("   ⚔️ درع باكير v4.0 | النسخة الدفاعية السيادية ");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("  • bakir-shield -all on/off      : التحكم الشامل");
    println!("  • bakir-shield -guard           : الحارس الصامت");
    println!("  • bakir-shield -stop            : إيقاف الحارس");
    println!("  • bakir-shield -redbutton       : الطوارئ الذكي");
    println!("  • bakir-shield -status          : التقرير الأمني");
    println!("  • bakir-shield -clean           : تصفير الحظر");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}
