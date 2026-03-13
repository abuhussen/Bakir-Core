use std::fs::{OpenOptions};
use std::io::Write;
use std::process::Command;
use std::env;
use colored::*;

fn main() {
    // 🎨 لوحة ألوان سايبربانك الفسفورية والمضيئة (إصدار النيون الأسطوري)
    // تم استخدام أكواد الألوان (256-bit) لضمان "التوهج" (Glowing effect) في الترمنال
    let luminous_cyan = "1;38;5;51";   // السيان الفسفوري المضيء جداً (بديل الأبيض) - للكتابة
    let luminous_turq = "1;38;5;121";  // الفيروزي المضيء المشع (بديل الزهري) - للأشياء الثابتة
    let purple = "1;38;5;129";         // البنفسجي العميق (للتمييز)
    let neon_yellow = "1;38;5;226";    // الأصفر الكهربائي (للتنبيهات)

    let user_name = env::var("USER").unwrap_or_else(|_| "User".to_string());

    // ─── مخرجات الترمنال أثناء التنفيذ ───
    println!("{}", "┌──────────────────────────────────────────────┐".magenta());
    println!("│      {}            │", "Bakir-Linux Neon Theme v1.0.1".bold().cyan());
    println!("{}", "└──────────────────────────────────────────────┘".magenta());
    println!("\n{} {}", "⚡ جاري حقن الرؤية البصرية النيونية لـ".bold().bright_cyan(), user_name.bold().magenta());

    // ─── إعداد محتوى ملف الـ .bashrc للمستخدم ───
    // تم تطبيق الألوان المضيئة بدقة على التنسيق الأصلي: السيان للكتابة، الفيروزي للأقواس والثوابت
    let theme_content = format!(
        "# --- Bakir-Linux Neon Theme (v1.0.1) ---\n\
export HISTCONTROL=ignoreboth\n\
shopt -s checkwinsize\n\
C_CYAN='\\[\\e[{l_cyan}m\\]'\n\
C_TURQ='\\[\\e[{l_turq}m\\]'\n\
C_PURP='\\[\\e[{purp}m\\]'\n\
C_YLW='\\[\\e[{ylw}m\\]'\n\
PS1=\"${{C_TURQ}}┌──(${{C_CYAN}}\\u${{C_TURQ}})  ${{C_YLW}}» ${{C_PURP}}B ${{C_YLW}}«  ${{C_TURQ}}(${{C_CYAN}}Bakir-Linux${{C_TURQ}})\\n${{C_TURQ}}└─${{C_YLW}}[${{C_CYAN}}\\w${{C_YLW}}] » ${{C_CYAN}}\"\n\
export LS_COLORS=\"di=1;38;5;51:ln=1;38;5;121:ex=1;38;5;226\"\n\
alias ls='ls --color=auto'\n\
trap 'echo -ne \"\\e[{l_cyan}m\"' DEBUG\n",
l_cyan=luminous_cyan, l_turq=luminous_turq, purp=purple, ylw=neon_yellow
    );

    // حقن الثيم للمستخدم الحالي (إضافة للكود)
    if let Ok(mut home_rc) = env::var("HOME").map(|h| format!("{}/.bashrc", h)) {
        inject_theme(&home_rc, &theme_content);
    }

    // حقن الثيم للروت (يتطلب صلاحيات sudo لتعديل /root/.bashrc)
    // للأمان، نقوم فقط بإنشاء ملف مؤقت ثم نسخه باستخدام sudo
    let root_cmd = format!("echo '{}' > /tmp/root_bashrc; sudo mv /tmp/root_bashrc /root/.bashrc", theme_content);
    let _ = Command::new("bash").arg("-c").arg(root_cmd).status();

    // ─── مخرجات الترمنال النهائية ───
    println!("{}", "\n[✔] تم حقن الهوية النيونية بنجاح!".green().bold());
    println!("{}", "⚠️  أغلق التيرمنال وافتحه مرة أخرى لتشهد التحول الأسطوري!".bright_magenta());
}

// دالة لإضافة الثيم لنهاية الملف (Append)
fn inject_theme(path: &str, content: &str) {
    if let Ok(mut file) = OpenOptions::new().append(true).create(true).open(path) {
        let _ = writeln!(file, "\n{}", content);
    }
}
