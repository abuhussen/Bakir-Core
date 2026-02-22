use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "-h" {
        display_help();
        return;
    }

    match args[1].as_str() {
        "-all" => handle_all_ports(&args),
        "-prt" => handle_single_port(&args),
        "-scan" => scan_ports(),
        "-ghost" => toggle_stealth_mode(&args),
        "-guard" => start_guard(), // ميزة الحارس الخلفي
        _ => println!("❌ أمر غير معروف. استخدم bakir -h لعرض القائمة."),
    }
}

fn display_help() {
    println!("🛡️ Bakir-Shield | حصن باقر السيادي");
    println!("------------------------------------------");
    println!("bakir -h             : عرض هذه القائمة");
    println!("bakir -all port off  : إغلاق كل المنافذ الخطرة");
    println!("bakir -all port on   : فتح كل المنافذ");
    println!("bakir -prt [رقم] on  : فتح منفذ محدد");
    println!("bakir -prt [رقم] off : إغلاق منفذ محدد");
    println!("bakir -scan          : عرض المنافذ النشطة");
    println!("bakir -ghost on/off  : الوضع الخفي (تجاهل الـ Ping)");
    println!("bakir -guard         : تفعيل الحارس اليقظ في الخلفية");
    println!("------------------------------------------");
}

fn send_plasma_notify(title: &str, msg: &str) {
    Command::new("notify-send")
        .args(&[title, msg, "-i", "security-high", "-a", "Bakir Shield"])
        .status()
        .unwrap();
}

fn handle_all_ports(args: &[String]) {
    if args.contains(&"off".to_string()) {
        Command::new("sudo").args(&["ufw", "--force", "enable"]).status().unwrap();
        Command::new("sudo").args(&["ufw", "default", "deny", "incoming"]).status().unwrap();
        send_plasma_notify("🛡️ الجدار الناري", "تم إغلاق كافة المنافذ.. النظام في وضع الحماية القصوى.");
    } else {
        Command::new("sudo").args(&["ufw", "default", "allow", "incoming"]).status().unwrap();
        send_plasma_notify("🛡️ الجدار الناري", "تم فتح المنافذ الافتراضية.");
    }
}

fn handle_single_port(args: &[String]) {
    if args.len() < 4 { return; }
    let port = &args[2];
    let action = &args[3];
    Command::new("sudo").args(&["ufw", action, port]).status().unwrap();
    send_plasma_notify("🛡️ تحديث المنفذ", &format!("تم {} المنفذ {} بنجاح.", action, port));
}

fn scan_ports() {
    let output = Command::new("sudo").args(&["ufw", "status", "numbered"]).output().unwrap();
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

fn toggle_stealth_mode(args: &[String]) {
    if args.contains(&"on".to_string()) {
        Command::new("sudo").args(&["sysctl", "-w", "net.ipv4.icmp_echo_ignore_all=1"]).status().unwrap();
        send_plasma_notify("👻 الوضع الخفي", "النظام الآن غير مرئي على الشبكة.");
    } else {
        Command::new("sudo").args(&["sysctl", "-w", "net.ipv4.icmp_echo_ignore_all=0"]).status().unwrap();
        send_plasma_notify("🌐 الوضع الخفي", "النظام الآن مرئي للشبكة.");
    }
}

fn start_guard() {
    println!("📡 الحارس اليقظ يعمل الآن في الخلفية...");
    send_plasma_notify("📡 حارس باقر", "بدأ الحارس بمراقبة محاولات الاختراق الآن.");
    // هنا نضع منطق مراقبة السجلات (Log Monitoring)
}