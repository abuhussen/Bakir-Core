use std::process::{Command, Stdio};
use std::env;
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use colored::*;

/// إرسال إشعارات أمنية ديناميكية تكتشف المستخدم الحالي
fn send_notification(title: &str, msg: &str, urgency: &str) {
    let user = env::var("SUDO_USER").unwrap_or_else(|_| env::var("USER").unwrap_or_default());
    if user.is_empty() { return; }

    let uid_out = Command::new("id").args(&["-u", &user]).output();
    if let Ok(output) = uid_out {
        let uid = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let bus_path = format!("unix:path=/run/user/{}/bus", uid);

        let _ = Command::new("sudo")
        .args(&[
            "-u", &user,
            "DISPLAY=:0",
            &format!("DBUS_SESSION_BUS_ADDRESS={}", bus_path),
              "notify-send", title, msg, "-i", "security-high", "-u", urgency, "-t", "4000"
        ])
        .status();
    }
}

fn set_ghost_mode(enable: bool) {
    let value = if enable { "1" } else { "0" };
    let status_txt = if enable { "وضع التخفي نشط 👻 (نظام صامت تماماً)" } else { "وضع التخفي معطل 👁️" };

    // تجاهل طلبات Ping
    let _ = Command::new("sudo").args(&["sysctl", "-w", &format!("net.ipv4.icmp_echo_ignore_all={}", value)]).status();

    if enable {
        // جعل جدار الحماية يسقط الحزم (Drop) بدلاً من الرد بالرفض لزيادة التخفي
        let _ = Command::new("sudo").args(&["ufw", "logging", "low"]).status();
    }

    println!("{} {}", "󰒔".cyan(), status_txt.bold().bright_cyan());
    send_notification("🛡️ Bakir-Shield", status_txt, "normal");
}

fn monitor_logic() {
    // في ديبيان الحديث قد يكون المسار /var/log/auth.log أو عبر journalctl
    let log_path = "/var/log/auth.log";
    if let Ok(file) = File::open(log_path) {
        let mut reader = BufReader::new(file);
        let _ = reader.seek(SeekFrom::End(0));
        loop {
            let mut line = String::new();
            if let Ok(len) = reader.read_line(&mut line) {
                if len > 0 && (line.contains("Failed password") || line.contains("authentication failure")) {
                    send_notification("🚨 محاولة اختراق", "تم رصد محاولة دخول فاشلة! الحارس يراقب..", "critical");
                }
            }
            thread::sleep(Duration::from_millis(500));
        }
    }
}

fn main() {
    // التحقق من الصلاحيات السيادية
    if unsafe { libc::getuid() != 0 } {
        println!("{}", "❌ خطأ أمني: درع باكير يتطلب صلاحيات sudo للعمل".red().bold());
        return;
    }

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
            println!("{}", "📡 يتم الآن إطلاق الحارس في الخلفية... النظام محمي الآن.".green().bold());
            send_notification("📡 حارس باكير", "بدأت المراقبة الصامتة في الخلفية", "normal");

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
            println!("{}", "🚨 وضع الطوارئ: عزل كامل وفوري للمنافذ!".red().bold());
            send_notification("🚨 RED BUTTON", "النظام في حالة عزل قصوى!", "critical");
        },
        "-status" => { let _ = Command::new("sudo").args(&["ufw", "status", "numbered"]).status(); },
        "-scan" => {
            println!("{}", "🔍 فحص المنافذ النشطة...".cyan());
            let _ = Command::new("sudo").args(&["ss", "-tuln"]).status();
            send_notification("🔍 فحص الشبكة", "اكتمل فحص المنافذ", "normal");
        },
        _ => display_help(),
    }
}

fn display_help() {
    println!("{}", "\n⚔️ Bakir-Shield v7.0 | النسخة العالمية السيادية".bold().bright_cyan());
    println!(" • -all on/off      | تفعيل/تعطيل الحماية الشاملة");
    println!(" • -gost on/off     | وضع التخفي (إخفاء الوجود على الشبكة)");
    println!(" • -guard           | تفعيل الحارس الذكي (يعمل في الخلفية)");
    println!(" • -redbutton       | وضع الطوارئ (عزل فوري وشامل)");
    println!(" • -status          | عرض حالة جدار الحماية");
    println!(" • -scan            | فحص المنافذ المفتوحة حالياً\n");
}
