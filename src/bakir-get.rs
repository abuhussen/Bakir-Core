use std::process::{Command, Stdio};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { display_help(); return; }
    
    match args[1].as_str() {
        "-v" => download_media(&args, "video"),
        "-a" => download_media(&args, "audio"),
        "-f" => download_file(&args),
        _ => display_help(),
    }
}

fn display_help() {
    println!("📥 bakir-get | المحرك السيادي (الإصدار المستقر)");
    println!("------------------------------------------");
    println!("bakir-get -v [الرابط]  : تحميل فيديو");
    println!("bakir-get -a [الرابط]  : تحميل صوت MP3");
    println!("bakir-get -f [الرابط]  : تحميل ملفات (صور، برامج، مضغوطة)");
}

fn send_notify(title: &str, msg: &str) {
    let _ = Command::new("notify-send").args(&["-i", "download", title, msg]).status();
}

fn download_media(args: &[String], mode: &str) {
    if args.len() < 3 { return; }
    let url = &args[2];
    let download_path = format!("{}/Downloads", env::var("HOME").unwrap_or_else(|_| "/home/bakir".into()));
    let _ = fs::create_dir_all(&download_path);

    send_notify("Bakir-Get", "🚀 جاري التحميل...");

    let mut cmd = Command::new("yt-dlp");
    cmd.args(&["-o", &format!("{}/%(title)s.%(ext)s", download_path), "--no-check-certificates", url]);

    if mode == "audio" { cmd.args(&["-x", "--audio-format", "mp3"]); }

    let status = cmd.status().expect("Failed to execute yt-dlp");
    if status.success() { send_notify("Bakir-Get", "✅ تم التحميل في Downloads"); }
}

fn download_file(args: &[String]) {
    if args.len() < 3 { return; }
    let url = &args[2];
    let download_path = format!("{}/Downloads", env::var("HOME").unwrap_or_else(|_| "/home/bakir".into()));
    let _ = fs::create_dir_all(&download_path);

    send_notify("Bakir-Get", "⚡ تحميل صاروخي...");

    // استخدام -d لتحديد المجلد و --out لتسمية الملف تلقائياً
    let status = Command::new("aria2c")
        .args(&[
            "-d", &download_path,
            "--allow-overwrite=true",
            "--auto-file-renaming=true",
            url
        ])
        .status();

    if status.unwrap().success() {
        send_notify("Bakir-Get", "✅ تم التحميل بنجاح");
    } else {
        println!("❌ خطأ: لم يتمكن المحرك من الوصول للرابط.");
    }
}