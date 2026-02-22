use std::process::{Command, Stdio};
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.contains(&"-h".to_string()) {
        display_help();
        return;
    }
    check_dependencies();

    match args[1].as_str() {
        "-v" => download_media(&args, "video"),
        "-a" => download_media(&args, "audio"),
        "-f" => download_file(&args),
        _ => println!("❌ أمر غير معروف. استخدم bakir-get -h للمساعدة."),
    }
}

fn display_help() {
    println!("📥 bakir-get | محرك التحميل السيادي لنظام باكير");
    println!("------------------------------------------");
    println!("bakir-get -v [الرابط]  : تحميل فيديو (MP4) مع تجاوز الحماية");
    println!("bakir-get -a [الرابط]  : تحميل مقطع صوتي فقط (MP3)");
    println!("bakir-get -f [الرابط]  : تحميل ملف مباشر بسرعة صاروخية");
    println!("------------------------------------------");
}

fn check_dependencies() {
    // تصحيح اسم الحزمة من aria2c إلى aria2
    let deps = [("yt-dlp", "yt-dlp"), ("aria2c", "aria2"), ("ffmpeg", "ffmpeg")];
    for (bin, pkg) in deps.iter() {
        if Command::new(bin).arg("--version").stdout(Stdio::null()).stderr(Stdio::null()).status().is_err() {
            println!("⚠️ الحزمة {} مفقودة، جاري تجهيزها...", pkg);
            let _ = Command::new("sudo").args(&["apt", "update"]).status();
            let _ = Command::new("sudo").args(&["apt", "install", "-y", pkg]).status();
        }
    }
}

fn download_media(args: &[String], mode: &str) {
    if args.len() < 3 { return; }
    let url = &args[2];
    let download_dir = format!("{}/Downloads", env::var("HOME").unwrap_or_else(|_| "/home/bakir".into()));
    
    // إضافة User-Agent لتجاوز خطأ 403 Forbidden
    let mut cmd = Command::new("yt-dlp");
    cmd.current_dir(&download_dir);
    cmd.args(&["--user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36"]);
    cmd.args(&["--no-check-certificates", "--update"]); // تحديث تلقائي للمحرك

    if mode == "video" {
        cmd.args(&["-f", "bestvideo+bestaudio/best", "--merge-output-format", "mp4", url]);
    } else {
        cmd.args(&["-x", "--audio-format", "mp3", url]);
    }

    println!("🚀 جاري تجاوز الحماية والتحميل...");
    let _ = cmd.status();
}

fn download_file(args: &[String]) {
    if args.len() < 3 { return; }
    let url = &args[2];
    let download_dir = format!("{}/Downloads", env::var("HOME").unwrap_or_else(|_| "/home/bakir".into()));

    // استخدام محرك aria2c مع إعدادات السرعة القصوى
    let _ = Command::new("aria2c")
        .args(&["-d", &download_dir, "--max-connection-per-server=16", "--split=16", url])
        .status();
}