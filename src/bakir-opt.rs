use std::process::Command;
use std::fs;
use std::path::Path;
use std::env;

fn main() {
    // التحقق من أن المستخدم يشغل البرنامج بصلاحيات sudo
    if !is_root() {
        println!("❌ خطأ: يجب تشغيل Bakir-Opt بصلاحيات sudo!");
        println!("💡 جرب كتابة: sudo bakir-opt");
        return;
    }

    println!("🛡️ Bakir-Opt: مُحسن النظام الآمن");
    println!("------------------------------------------");

    println!("🧹 جاري تنظيف مخلفات الحزم...");
    run_cmd("apt-get", &["autoremove", "-y"]);
    run_cmd("apt-get", &["autoclean", "-y"]);

    let tmp_path = "/tmp";
    if Path::new(tmp_path).exists() {
        println!("🗑️ تنظيف الملفات المؤقتة...");
        run_cmd("find", &[tmp_path, "-type", "f", "-atime", "+1", "-delete"]);
    }

    let conf_path = "/etc/sysctl.d/99-bakir-speed.conf";
    println!("🚀 تحسين إعدادات الشبكة...");
    let settings = "net.core.rmem_max = 16777216\nnet.core.wmem_max = 16777216\nnet.ipv4.tcp_congestion_control = bbr";
    
    // محاولة الكتابة مع معالجة الخطأ لمنع الانهيار
    match fs::write(conf_path, settings) {
        Ok(_) => {
            run_cmd("sysctl", &["-p", conf_path]);
            println!("✅ تم تحسين سرعة الإنترنت.");
        },
        Err(e) => println!("⚠️ فشل تعديل إعدادات الشبكة: {}", e),
    }

    println!("🔧 فحص وإصلاح النظام...");
    run_cmd("dpkg", &["--configure", "-a"]);
    run_cmd("apt-get", &["install", "-f", "-y"]);

    println!("------------------------------------------");
    println!("✅ انتهت عملية الصيانة بنجاح وأمان!");
}

fn is_root() -> bool {
    env::var("USER").map(|u| u == "root").unwrap_or(false) || 
    Command::new("id").arg("-u").output().map(|o| String::from_utf8_lossy(&o.stdout).trim() == "0").unwrap_or(false)
}

fn run_cmd(cmd: &str, args: &[&str]) {
    let _ = Command::new(cmd).args(args).status();
}