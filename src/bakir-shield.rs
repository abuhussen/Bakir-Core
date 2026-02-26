use std::process::{Command};
use std::env;
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;

fn send_bakir_notification(title: &str, message: &str, urgency: &str) {
    let _ = Command::new("sudo")
        .args(&["-u", "bakir", "DISPLAY=:0", "DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus", 
                "notify-send", title, message, "-i", "security-high", "-u", urgency])
        .status();
}

fn exterminate(target_ip: &str) {
    send_bakir_notification("💀 إبادة سيادية", &format!("تم سحق هجوم من {}. جاري تدمير المعتدي!", target_ip), "critical");
    let _ = Command::new("sudo").args(&["iptables", "-I", "INPUT", "-s", target_ip, "-j", "DROP"]).status();
    let _ = Command::new("sudo").args(&["hping3", "--flood", "-S", "-p", "80", target_ip]).spawn();
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

    println!("📡 القناص الآلي نشط الآن... الحدود تحت السيطرة.");
    send_bakir_notification("📡 حارس باكير", "بدأ القناص الآلي المراقبة.", "normal");

    loop {
        let mut line = String::new();
        if let Ok(len) = reader.read_line(&mut line) {
            if len > 0 && line.contains("Failed password") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                for (i, part) in parts.iter().enumerate() {
                    if *part == "from" && i + 1 < parts.len() {
                        exterminate(parts[i+1]);
                    }
                }
            }
        }
        thread::sleep(Duration::from_millis(500));
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
                send_bakir_notification("🛡️ حصن باكير", "تم فتح جميع القنوات السيادية.", "normal");
                println!("🚀 تم استعادة الاتصالات بالكامل.");
            } else {
                let _ = Command::new("sudo").args(&["ufw", "default", "deny", "incoming"]).status();
                let _ = Command::new("sudo").args(&["ufw", "--force", "enable"]).status();
                println!("🔒 تم إغلاق المنافذ الواردة.");
            }
        },
        "-ghost" => {
            let val = if args.contains(&"on".to_string()) { "1" } else { "0" };
            let _ = Command::new("sudo").args(&["sysctl", "-w", &format!("net.ipv4.icmp_echo_ignore_all={}", val)]).status();
            send_bakir_notification("👻 وضع الشبح", if val == "1" { "النظام متخفٍ." } else { "النظام مرئي." }, "normal");
        },
        "-guard" => monitor_logs(),
        "-redbutton" => {
            let _ = Command::new("sudo").args(&["ufw", "default", "deny", "outgoing"]).status();
            let _ = Command::new("sudo").args(&["ufw", "default", "deny", "incoming"]).status();
            let _ = Command::new("sudo").args(&["ufw", "--force", "enable"]).status();
            send_bakir_notification("🚨 زر الطوارئ", "تم عزل النظام بالكامل!", "critical");
            println!("🚨 وضع العزل نشط.");
        },
        _ => display_help(),
    }
}

fn display_help() {
    println!("⚔️ درع باكير النهائي v3.2 | Bakir-Shield");
    println!(" • bakir-shield -all on/off      : التحكم الشامل");
    println!(" • bakir-shield -ghost on/off    : وضع الشبح");
    println!(" • bakir-shield -guard           : تفعيل القناص");
    println!(" • bakir-shield -redbutton       : زر الإبادة (العزل)");
}
