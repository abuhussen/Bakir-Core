use std::process::{Command, Stdio};
use std::env;
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};

fn send_bakir_notification(title: &str, message: &str, urgency: &str) {
    let _ = Command::new("sudo")
        .args(&["-u", "bakir", "DISPLAY=:0", "DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus", 
                "notify-send", title, message, "-i", "security-high", "-u", urgency])
        .status();
}

// وظيفة "الإبادة والتلغيم الحراري"
fn exterminate(target_ip: &str) {
    println!("🚨 [إنذار قتالي] رصد متسلل: {}. جاري سحق الهدف...", target_ip);
    send_bakir_notification("💀 إبادة سيادية", &format!("رصد هجوم من {}. جاري تدمير جهاز المعتدي!", target_ip), "critical");
    
    // 1. الحظر الفوري في الجدار
    let _ = Command::new("sudo").args(&["iptables", "-I", "INPUT", "-s", target_ip, "-j", "DROP"]).status();
    
    // 2. الهجوم المضاد (قنبلة الحزم - Flooding) لإجهاد معالج المهاجم
    let _ = Command::new("sudo").args(&["hping3", "--flood", "-S", "-p", "80", target_ip]).spawn();
}

fn monitor_logs() {
    println!("📡 القناص الآلي نشط... يراقب محاولات الاختراق في صمت.");
    let file = File::open("/var/log/auth.log").expect("❌ لا يمكن الوصول للسجلات - تأكد من صلاحيات sudo");
    let mut reader = BufReader::new(file);
    
    // الانتقال لآخر الملف لتجنب معالجة الهجمات القديمة
    let _ = reader.seek(SeekFrom::End(0));

    loop {
        let mut line = String::new();
        let resp = reader.read_line(&mut line).unwrap();
        if resp > 0 {
            // رصد محاولات دخول فاشلة (SSH أو Sudo)
            if line.contains("Failed password") || line.contains("authentication failure") {
                // استخراج الـ IP (تبسيطاً سنقوم بالبحث عن الأنماط الشائعة)
                let parts: Vec<&str> = line.split_whitespace().collect();
                for (i, part) in parts.iter().enumerate() {
                    if *part == "from" && i + 1 < parts.len() {
                        let ip = parts[i+1];
                        exterminate(ip);
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
        "-guard" => {
            monitor_logs();
        },
        "-redbutton" => {
            println!("🚨 تفعيل زر الطوارئ! عزل شامل...");
            let _ = Command::new("sudo").args(&["ufw", "deny", "out", "to", "any"]).status();
            let _ = Command::new("sudo").args(&["ufw", "deny", "in", "from", "any"]).status();
            send_bakir_notification("🚨 زر الطوارئ", "النظام في وضع العزل المطلق!", "critical");
        },
        "-all" => {
            let _ = Command::new("sudo").args(&["ufw", "default", "allow", "incoming"]).status();
            send_bakir_notification("🛡️ حصن باكير", "تم فتح القنوات السيادية.", "normal");
        },
        _ => display_help(),
    }
}

fn display_help() {
    println!("⚔️ درع باكير العسكري v2.7 | القناص الآلي");
    println!(" • bakir-shield -guard      : تفعيل القناص والردع التلقائي");
    println!(" • bakir-shield -redbutton  : زر الإبادة (عزل النظام)");
    println!(" • bakir-shield -all on     : إعادة التشغيل الطبيعي");
}
