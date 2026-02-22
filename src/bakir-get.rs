use std::process::{Command, Stdio};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { display_help(); return; }
    
    // تأمين الأدوات قبل البدء
    check_dependencies();

    match args[1].as_str() {
        "-v" => download_media(&args, "video"),
        "-a" => download_media(&args, "audio"),
        "-f" => download_file(&args),
        _ => display_help(),
    }
}

fn display_help() {
    println!("📥 bakir-get | محرك التحميل السيادي المحسن");
    println!("------------------------------------------");
    println!("bakir-get -v [الرابط]  : تحميل فيديو عالي الجودة");
    println!("bakir-get -f [الرابط]  : تحميل ملف مباشر (سريع)");
}

fn check_dependencies() {
    // تثبيت libnotify-bin فوراً لضمان عمل الإشعارات
    let _ = Command::new("sudo").args(&["apt", "install", "-y", "libnotify-bin", "aria2", "ffmpeg"]).status();
}

fn send_notify(title: &str, msg: &str) {
    let _ = Command::new("notify-send")
        .args(&["-i", "download", title, msg])
        .status();
}

fn download_media(args: &[String], mode: &str) {
    if args.len() < 3 { return; }
    let url = &args[2];
    send_notify("Bakir-Get", "🚀 جاري سحب الوسائط وتجاوز الحماية...");

    // تحديث yt-dlp لأحدث نسخة عالمية مباشرة (تجاوز مستودعات ديببيان القديمة)
    let _ = Command::new("sudo").args(&["wget", "https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp", "-O", "/usr/local/bin/yt-dlp"]).status();
    let _ = Command::new("sudo").args(&["chmod", "a+rx", "/usr/local/bin/yt-dlp"]).status();

    let mut cmd = Command::new("yt-dlp");
    cmd.args(&["--user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) Chrome/122.0.0.0 Safari/537.36"]);
    cmd.args(&["--no-check-certificates", "-f", "bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best", url]);

    if mode == "audio" {
        cmd.args(&["-x", "--audio-format", "mp3"]);
    }

    if cmd.status().unwrap().success() {
        send_notify("Bakir-Get", "✅ اكتمل تحميل الوسائط بنجاح");
    } else {
        send_notify("Bakir-Get", "❌ فشل التحميل.. تحقق من الرابط");
    }
}

fn download_file(args: &[String]) {
    if args.len() < 3 { return; }
    let url = &args[2];
    send_notify("Bakir-Get", "⚡ جاري التحميل الصاروخي للملف...");

    let status = Command::new("aria2c")
        .args(&[
            "--user-agent=Mozilla/5.0", 
            "--max-connection-per-server=5", 
            "--continue=true",
            url
        ])
        .status();

    if status.unwrap().success() {
        send_notify("Bakir-Get", "✅ تم تحميل الملف بنجاح");
    }
}