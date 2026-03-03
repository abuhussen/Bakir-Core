use std::process::Command;
use std::env;
use std::thread;
use std::time::Duration;
use colored::*;

fn send_notification(title: &str, msg: &str) {
    // إرسال إشعار للنظام يظهر للمستخدم حتى تحت sudo
    let _ = Command::new("notify-send")
        .args(&[
            title, 
            msg, 
            "-i", "system-software-update",
            "-t", "5000"
        ])
        .status();
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
    // التحقق من الصلاحيات السيادية (Root)
    let output = Command::new("id").arg("-u").output().unwrap();
    let uid = String::from_utf8_lossy(&output.stdout).trim().parse::<u32>().unwrap();

    if uid != 0 {
        println!("{}", "❌ خطأ سيادي: يجب تشغيل الوحش بصلاحيات sudo".red().bold());
        return;
    }

    println!("{}", "--------------------------------------------------".cyan());
    println!("{}", "🔥 Bakir-Opt v7.0 | النسخة الوحشية (الترميم الجراحي)".bold().bright_red());
    println!("{}", "--------------------------------------------------".cyan());

    send_notification("🔥 Bakir-Opt", "بدأت عملية الترميم الجراحي للنظام...");

    let steps = vec![
        ("الترميم الذاتي", "apt-get install -f -y", "تم إصلاح الحزم المكسورة بنجاح"),
        ("تطهير الكاش", "sync; echo 3 > /proc/sys/vm/drop_caches", "تم تصفير ذاكرة النظام المؤقتة"),
        ("التنظيف العميق", "apt-get autoremove -y && apt-get clean", "تم حذف مخلفات النظام بنجاح"),
        ("ضغط السجلات", "journalctl --vacuum-time=2d", "تم تنظيف سجلات النظام القديمة"),
        ("تحسين القرص", "fstrim -av", "تم تحسين أداء قرص SSD بنجاح"),
    ];

    for step in steps {
        execute_step(step.0, step.1, step.2);
        thread::sleep(Duration::from_millis(800));
    }

    println!("{}", "--------------------------------------------------".cyan());
    println!("{}", "📋 ملخص العملية الجراحية:".bold().white());
    println!("{}", "✅ تم صيانة نظام باكير بنجاح.".green());
    println!("{}", "🚀 النظام الآن في قمة عطائه يا سيادة المستشار.".bright_cyan());
    println!("{}", "--------------------------------------------------".cyan());

    send_notification("✅ اكتمل الترميم", "النظام الآن في حالة مثالية!");
}
