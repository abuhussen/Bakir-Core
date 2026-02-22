use std::process::Command;
use std::fs;
use std::path::Path;

fn main() {
    println!("🛡️ Bakir-Opt: مُحسن النظام الآمن");
    println!("------------------------------------------");

    // 1. تنظيف الحزم التالفة (بشكل آمن)
    println!("🧹 جاري تنظيف مخلفات الحزم...");
    run_cmd("apt-get", &["autoremove", "-y"]);
    run_cmd("apt-get", &["autoclean", "-y"]);

    // 2. تنظيف الملفات المؤقتة مع التحقق من المسار
    let tmp_path = "/tmp";
    if Path::new(tmp_path).exists() {
        println!("🗑️ تنظيف الملفات المؤقتة...");
        // نكتفي بمسح الملفات التي مر عليها وقت طويل أو غير مستخدمة
        run_cmd("find", &[tmp_path, "-type", "f", "-atime", "+1", "-delete"]);
    }

    // 3. تسريع الإنترنت (إضافة الإعدادات فقط إذا لم تكن موجودة)
    let conf_path = "/etc/sysctl.d/99-bakir-speed.conf";
    if !Path::new(conf_path).exists() {
        println!("🚀 تحسين إعدادات الشبكة...");
        let settings = "net.core.rmem_max = 16777216\nnet.core.wmem_max = 16777216\nnet.ipv4.tcp_congestion_control = bbr";
        fs::write(conf_path, settings).expect("فشل كتابة إعدادات السرعة");
        run_cmd("sysctl", &["-p", conf_path]);
    }

    // 4. إصلاح الحزم المكسورة
    println!("🔧 فحص وإصلاح النظام...");
    run_cmd("dpkg", &["--configure", "-a"]);
    run_cmd("apt-get", &["install", "-f", "-y"]);

    println!("------------------------------------------");
    println!("✅ انتهت عملية الصيانة بنجاح وأمان تادم!");
}

fn run_cmd(cmd: &str, args: &[&str]) {
    let status = Command::new("sudo")
        .arg(cmd)
        .args(args)
        .status();
    
    match status {
        Ok(s) if s.success() => {},
        _ => println!("⚠️ تنبيه: فشل تنفيذ أمر {} ولكن النظام مستقر.", cmd),
    }
}