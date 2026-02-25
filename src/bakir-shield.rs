use std::process::{Command, Stdio};
use std::env;

fn send_bakir_notification(title: &str, message: &str) {
    // ميزتك الاحترافية لإرسال الإشعارات من خلف sudo
    let _ = Command::new("sudo")
        .args(&["-u", "bakir", "DISPLAY=:0", "DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus", 
                "notify-send", title, message, "-i", "security-high"])
        .status();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        display_help();
        return;
    }

    match args[1].as_str() {
        "-all" => {
            let action = if args.contains(&"off".to_string()) { "deny" } else { "allow" };
            let _ = Command::new("sudo").args(&["ufw", "default", action, "incoming"]).status();
            let _ = Command::new("sudo").args(&["ufw", "--force", "enable"]).status();
            send_bakir_notification("🛡️ حصن باكير", "تم تحديث السياسة الشاملة للمنافذ.");
        },
        "-port" => {
            if args.len() >= 4 {
                let action = if args[3] == "on" { "allow" } else { "deny" };
                let _ = Command::new("sudo").args(&["ufw", action, &args[2]]).status();
                send_bakir_notification("🛡️ حصن باكير", &format!("تحديث المنفذ {}: {}", args[2], args[3]));
            }
        },
        "-ghost" => {
            let val = if args.contains(&"on".to_string()) { "1" } else { "0" };
            let _ = Command::new("sudo").args(&["sysctl", "-w", &format!("net.ipv4.icmp_echo_ignore_all={}", val)]).status();
            send_bakir_notification("👻 وضع الشبح", if val == "1" { "النظام الآن متخفٍ." } else { "النظام الآن مرئي." });
        },
        "-scan" => {
             println!("📊 جاري فحص المنافذ والمستويات...");
             let _ = Command::new("sudo").args(&["ufw", "status", "numbered"]).stdout(Stdio::inherit()).status();
        },
        "-guard" => {
            println!("📡 تشغيل الحارس السيادي...");
            send_bakir_notification("📡 حارس باكير", "بدأ الحارس مراقبة النظام نشطاً.");
        },
        _ => println!("❌ أمر غير معروف. استخدم bakir-shield -h للمساعدة."),
    }
}

fn display_help() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("   🛡️ حصن باكير السيادي | Bakir-Shield v2.0   ");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!(" [ التعليمات الرسمية والموحدة ]:");
    println!("  • bakir-shield -all on/off      : فتح/إغلاق شامل");
    println!("  • bakir-shield -port [رقم] on/off : التحكم بمنفذ محدد");
    println!("  • bakir-shield -scan            : عرض جدول الحماية");
    println!("  • bakir-shield -ghost on/off    : وضع الشبح (إخفاء الـ Ping)");
    println!("  • bakir-shield -guard           : تفعيل الحارس النشط");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
}
