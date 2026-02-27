use std::process::{Command};
use std::env;
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;

const WHITELIST: &[&str] = &["127.0.0.1", "192.168"];

fn send_notification(title: &str, msg: &str, urgency: &str) {
    // إصلاح ديناميكي للإشعارات لتعمل مع أي جلسة مستخدم
    let _ = Command::new("notify-send")
        .args(&[title, msg, "-i", "security-high", "-u", urgency])
        .env("DISPLAY", ":0")
        .env("DBUS_SESSION_BUS_ADDRESS", &format!("unix:path=/run/user/{}/bus", "1000"))
        .status();
}

fn defense_action(target_ip: &str) {
    for white_ip in WHITELIST {
        if target_ip.starts_with(white_ip) { return; }
    }
    send_notification("🛡️ تأمين باكير", &format!("تم حظر المعتدي: {}", target_ip), "normal");
    let _ = Command::new("sudo").args(&["iptables", "-A", "INPUT", "-s", target_ip, "-j", "DROP"]).status();
}

fn monitor_logs() {
    send_notification("📡 حارس باكير", "الدرع الدفاعي نشط الآن ويراقب بصمت.", "normal");
    let log_path = "/var/log/auth.log";
    if !Path::new(log_path).exists() {
        let _ = Command::new("sudo").args(&["touch", log_path]).status();
    }
    
    let file = File::open(log_path).expect("❌ فشل الوصول للسجلات!");
    let mut reader = BufReader::new(file);
    let _ = reader.seek(SeekFrom::End(0));

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
                let _ = Command::new("sudo").args(&["ufw", "default", "allow", "incoming"]).status();
                let _ = Command::new("sudo").args(&["ufw", "default", "allow", "outgoing"]).status();
                let _ = Command::new("sudo").args(&["ufw", "--force", "enable"]).status();
                send_notification("🚀 استعادة كاملة", "الشبكة مفتوحة ومؤمنة بالدرع.", "normal");
            } else {
                let _ = Command::new("sudo").args(&["ufw", "default", "deny", "incoming"]).status();
                send_notification("🔒 إغلاق المنافذ", "تم منع الاتصالات الواردة غير المصرح بها.", "normal");
            }
        },
        "-guard" => monitor_logs(),
        "-stop" => {
            // تنظيف كامل وإعادة الاتصال
            let _ = Command::new("sudo").args(&["ufw", "disable"]).status();
            let _ = Command::new("sudo").args(&["iptables", "-F"]).status();
            let _ = Command::new("sudo").args(&["pkill", "-f", "bakir-shield"]).status();
            send_notification("🛑 إيقاف الدرع", "تم إيقاف كافة العمليات الدفاعية وفتح الشبكة.", "normal");
        },
        "-redbutton" => {
            let ports = ["21", "22", "23", "445"];
            for port in &ports {
                let _ = Command::new("sudo").args(&["ufw", "deny", port]).status();
            }
            send_notification("🚨 حماية ذكية", "تم إغلاق المنافذ الحساسة، تصفحك آمن.", "critical");
        },
        "-clean" => {
            let _ = Command::new("sudo").args(&["iptables", "-F"]).status();
            send_notification("🧹 تنظيف", "تم مسح قائمة الحظر السوداء.", "normal");
        },
        "-status" => {
            println!("📊 حالة درع باكير v4.1:");
            let _ = Command::new("sudo").args(&["iptables", "-L", "INPUT", "-v", "-n"]).status();
        },
        _ => display_help(),
    }
}

fn display_help() {
    println!("⚔️ Bakir-Shield v4.1 | المصححة");
    println!(" • bakir-shield -all on/off");
    println!(" • bakir-shield -guard");
    println!(" • bakir-shield -stop");
    println!(" • bakir-shield -redbutton");
}
