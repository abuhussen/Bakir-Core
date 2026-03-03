use std::process::Command;
use std::env;
use std::thread;
use std::time::Duration;
use colored::*;

fn send_notification(title: &str, msg: &str) {
    // تحديد المستخدم الحقيقي الذي استخدم sudo لإرسال الإشعار لسطح مكتبه
    let user = env::var("SUDO_USER").unwrap_or_else(|_| env::var("USER").unwrap_or_default());
    if !user.is_empty() {
        let _ = Command::new("sudo")
            .args(&[
                "-u", &user, 
                "DISPLAY=:0", 
                "DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus", // مسار افتراضي لمستخدم ديبيان الأول
                "notify-send", title, msg, "-i", "system-software-update"
            ])
            .status();
    }
}

fn execute_step(name: &str, cmd: &str, success_msg: &str) -> bool {
    println!("⏳ جاري {}...", name.yellow().bold());
    let status = Command::new("sh").arg("-c").arg(cmd).status();
    match status {
        Ok(s) if s.success() => {
            println!("✅ {}", success_msg.green());
            true
        }
        _ => {
            println!("⚠️ {} {}", "فشل في".red(), name);
            false
        }
    }
}

fn main() {
    let output = Command::new("id").arg("-u").output().unwrap();
    let uid = String::from_utf8_lossy(&output.stdout).trim().parse::<u32>().unwrap();

    if uid != 0 {
        println!("{}", "❌ خطأ سيادي: يجب تشغيل الوحش بصلاحيات sudo".red().bold());
        return;
    }

    println!("{}", "--------------------------------------------------".cyan());
    println!("{}", "🔥 Bakir-Opt v7.1 | النسخة الوحشية المحدثة".bold().bright_red());
    println!("{}", "--------------------------------------------------".cyan());

    send_notification("🔥 Bakir-Opt", "بدأت عملية الترميم الجراحي...");

    let steps = vec![
        ("الترميم الذاتي", "apt-get install -f -y", "تم إصلاح الحزم المكسورة"),
        ("تطهير الكاش", "sync; echo 3 > /proc/sys/vm/drop_caches", "تم تصفير ذاكرة النظام"),
        ("التنظيف العميق", "apt-get autoremove -y && apt-get clean", "تم حذف مخلفات النظام"),
        ("تحسين القرص", "fstrim -av", "تم تحسين أداء SSD"),
    ];

    for step in steps {
        execute_step(step.0, step.1, step.2);
        thread::sleep(Duration::from_millis(500));
    }

    println!("{}", "--------------------------------------------------".cyan());
    println!("{}", "🚀 النظام الآن في قمة عطائه!".bright_cyan());
    send_notification("✅ اكتمل الترميم", "النظام الآن في حالة مثالية!");
}
