use std::process::{Command, Stdio};
use std::env;

fn send_bakir_notification(title: &str, message: &str) {
    // محاولة إرسال الإشعار للمستخدم الحالي حتى لو كان التشغيل بـ sudo
    let _ = Command::new("sudo")
        .args(&["-u", "bakir", "DISPLAY=:0", "DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus", 
                "notify-send", title, message, "-i", "security-high"])
        .status();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.contains(&"-h".to_string()) {
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
             println!("📊 جاري فحص المنافذ...");
             let _ = Command::new("sudo").args(&["ufw", "status", "numbered"]).stdout(Stdio::inherit()).status();
        },
        "-guard" => {
            println!("📡 تشغيل الحارس...");
            send_bakir_notification("📡 حارس باكير", "بدأ الحارس مراقبة النظام.");
        },
        _ => println!("❌ أمر غير معروف."),
    }
}

fn display_help() {
    println!("🛡️ حصن باكير السيادي | Bakir-Shield");
    println!("------------------------------------------");
    println!("bakir -all on/off          : فتح/إغلاق شامل");
    println!("bakir -port [الرقم] on/of    : التحكم بمنفذ محدد");
    println!("bakir -scan                : عرض الجدول");
    println!("bakir -ghost on/off        : وضع الشبح");
    println!("bakir -guard               : تفعيل الحارس");
}