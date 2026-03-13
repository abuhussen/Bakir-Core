use std::process::Command;
use std::env;
use std::thread;
use std::time::Duration;
use colored::*;

/// وظيفة إرسال الإشعارات لسطح المكتب بطريقة احترافية ديناميكية
fn send_notification(title: &str, msg: &str) {
    let user = env::var("SUDO_USER").unwrap_or_else(|_| env::var("USER").unwrap_or_default());

    if !user.is_empty() {
        let uid_out = Command::new("id").args(&["-u", &user]).output();
        if let Ok(output) = uid_out {
            let uid = String::from_utf8_lossy(&output.stdout).trim().to_string();
            let bus_path = format!("unix:path=/run/user/{}/bus", uid);

            let _ = Command::new("sudo")
            .args(&[
                "-u", &user,
                "DISPLAY=:0",
                &format!("DBUS_SESSION_BUS_ADDRESS={}", bus_path),
                  "notify-send", title, msg, "-i", "system-software-update"
            ])
            .status();
        }
    }
}

/// تنفيذ الأوامر البرمجية مع فحص حالة الخروج
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
    // التحقق الصارم من الصلاحيات باستخدام libc
    if unsafe { libc::getuid() != 0 } {
        println!("{}", "❌ خطأ سيادي: يجب تشغيل Bakir-Opt بصلاحيات sudo".red().bold());
        return;
    }

    println!("{}", "==================================================".cyan());
    println!("{}", "🔥 Bakir-Opt v7.1 | النسخة الاحترافية المحصنة".bold().bright_red());
    println!("{}", "==================================================".cyan());

    send_notification("🔥 Bakir-Opt", "بدأت عملية الترميم والتحسين الجراحي...");

    let steps = [
        ("إصلاح الحزم", "apt-get install -f -y", "تم ترميم الحزم المكسورة بنجاح"),
        ("تطهير الذاكرة", "sync; echo 3 > /proc/sys/vm/drop_caches", "تم تصفير كاش النظام بنجاح"),
        ("التنظيف العميق", "apt-get autoremove -y && apt-get clean", "تم التخلص من النفايات البرمجية"),
        ("تحسين أداء الأقراص", "fstrim -av", "تم تحسين استجابة وحدات التخزين SSD"),
    ];

    for (name, cmd, msg) in steps.iter() {
        execute_step(name, cmd, msg);
        thread::sleep(Duration::from_millis(600));
    }

    println!("{}", "==================================================".cyan());
    println!("{}", "🚀 Bakir-Linux الآن في حالة الأداء القصوى!".bright_cyan());
    send_notification("✅ اكتمل الترميم", "نظامك الآن جاهز للعمل الشاق!");
}
