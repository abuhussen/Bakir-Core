use std::process::{Command, Stdio};
use std::env;
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use colored::*;

fn send_notification(title: &str, msg: &str, urgency: &str) {
    let _ = Command::new("notify-send")
        .args(&[title, msg, "-i", "security-high", "-u", urgency, "-t", "4000"])
        .status();
}

fn set_ghost_mode(enable: bool) {
    let value = if enable { "1" } else { "0" };
    let status_txt = if enable { "وضع التخفي نشط 👻" } else { "وضع التخفي معطل 👁️" };
    let _ = Command::new("sudo").args(&["sysctl", "-w", &format!("net.ipv4.icmp_echo_ignore_all={}", value)]).status();
    println!("{} {}", "󰒔".cyan(), status_txt.bold().bright_cyan());
    send_notification("🛡️ Bakir-Shield", status_txt, "normal");
}

fn monitor_logic() {
    let log_path = "/var/log/auth.log";
    if let Ok(file) = File::open(log_path) {
        let mut reader = BufReader::new(file);
        let _ = reader.seek(SeekFrom::End(0));
        loop {
            let mut line = String::new();
            if let Ok(len) = reader.read_line(&mut line) {
                if len > 0 && line.contains("Failed password") {
                    send_notification("🚨 محاولة اختراق", "تم رصد محاولة دخول فاشلة وحظرها!", "critical");
                }
            }
            thread::sleep(Duration::from_millis(500));
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { display_help(); return; }

    match args[1].as_str() {
        "-all" => {
            let on = args.len() > 2 && args[2] == "on";
            let action = if on { "enable" } else { "disable" };
            let policy = if on { "deny" } else { "allow" };
            let _ = Command::new("sudo").args(&["ufw", "default", policy, "incoming"]).status();
            let _ = Command::new("sudo").args(&["ufw", action]).status();
            let msg = if on { "الحماية القصوى نشطة 🛡️" } else { "تم تعطيل الحماية ⚠️" };
            println!("{}", msg.yellow());
            send_notification("🛡️ Bakir-Shield", msg, "normal");
        },
        "-gost" => { if args.len() > 2 { set_ghost_mode(args[2] == "on"); } },
        "-guard" => {
            println!("{}", "📡 يتم الآن إطلاق الحارس في الخلفية... يمكنك إغلاق التيرمنال.".green().bold());
            send_notification("📡 حارس باكير", "بدأت المراقبة الصامتة في الخلفية", "normal");
            // إطلاق المراقبة في عملية منفصلة تماماً
            Command::new(env::current_exe().unwrap())
                .arg("--internal-monitor")
                .stdin(Stdio::null())
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("فشل إطلاق الحارس");
        },
        "--internal-monitor" => { monitor_logic(); },
        "-redbutton" => {
            let _ = Command::new("sudo").args(&["ufw", "default", "deny", "incoming"]).status();
            let _ = Command::new("sudo").args(&["ufw", "deny", "22"]).status();
            let _ = Command::new("sudo").args(&["ufw", "enable"]).status();
            println!("{}", "🚨 وضع الطوارئ: عزل كامل للمنافذ القادمة!".red().bold());
            send_notification("🚨 RED BUTTON", "النظام في حالة عزل قصوى!", "critical");
        },
        "-status" => { let _ = Command::new("sudo").args(&["ufw", "status", "numbered"]).status(); },
        "-scan" => {
            println!("{}", "🔍 فحص المنافذ...".cyan());
            let _ = Command::new("sudo").args(&["ss", "-tuln"]).status();
            send_notification("🔍 فحص الشبكة", "اكتمل فحص المنافذ", "normal");
        },
        _ => display_help(),
    }
}

fn display_help() {
    println!("{}", "\n⚔️ Bakir-Shield v7.0 | النسخة العالمية السيادية".bold().bright_cyan());
    println!(" • -all on/off      | تفعيل/تعطيل الحماية");
    println!(" • -gost on/off     | وضع التخفي (Ghost Mode)");
    println!(" • -guard           | تفعيل الحارس الصامت (خلفية)");
    println!(" • -redbutton       | وضع الطوارئ (عزل فوري)");
    println!(" • -status          | عرض الحالة");
    println!(" • -scan            | فحص المنافذ\n");
}
