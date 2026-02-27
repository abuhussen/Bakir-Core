use std::process::{Command};
use std::env;
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

fn send_notification(title: &str, msg: &str, urgency: &str) {
    let _ = Command::new("notify-send")
        .args(&[title, msg, "-i", "security-high", "-u", urgency])
        .status();
}

fn monitor_logs() {
    send_notification("📡 حارس باكير", "الدرع الدفاعي نشط ويراقب الآن.", "normal");
    let log_path = "/var/log/auth.log";
    let file = File::open(log_path).unwrap_or_else(|_| {
        Command::new("sudo").args(&["touch", log_path]).status().unwrap();
        File::open(log_path).expect("❌ فشل الوصول للسجلات")
    });
    let mut reader = BufReader::new(file);
    let _ = reader.seek(SeekFrom::End(0));
    loop {
        let mut line = String::new();
        if let Ok(len) = reader.read_line(&mut line) {
            if len > 0 && line.contains("Failed password") {
                // منطق الحماية هنا
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
            } else {
                let _ = Command::new("sudo").args(&["ufw", "default", "deny", "incoming"]).status();
            }
        },
        "-port" => {
            if args.len() > 3 {
                let action = if args[3] == "on" { "allow" } else { "deny" };
                let _ = Command::new("sudo").args(&["ufw", action, &args[2]]).status();
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
            send_notification("🚨 الطوارئ", "تم عزل المنافذ الحساسة.", "critical");
        },
        "-status" => {
            let _ = Command::new("sudo").args(&["ufw", "status", "numbered"]).status();
        },
        "-scan" => {
            let _ = Command::new("sudo").args(&["ss", "-tuln"]).status();
            send_notification("🔍 فحص", "اكتمل فحص الشبكة.", "normal");
        },
        _ => display_help(),
    }
}

fn display_help() {
    println!("⚔️ Bakir-Shield v4.2 | المستقر");
    println!(" • -all on/off | -port [P] on/off | -guard | -stop | -status | -scan");
}
