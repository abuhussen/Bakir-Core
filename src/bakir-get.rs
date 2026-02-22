use std::process::{Command, Stdio};
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { display_help(); return; }
    
    check_dependencies();

    match args[1].as_str() {
        "-v" => download_media(&args, "video"),
        "-a" => download_media(&args, "audio"),
        "-f" => download_file(&args),
        _ => display_help(),
    }
}

fn display_help() {
    println!("📥 bakir-get | المحرك السيادي للتحميل الشامل");
    println!("------------------------------------------");
    println!("bakir-get -v [الرابط]  : تحميل فيديو (يوتيوب، فيسبوك، تيك توك، إلخ)");
    println!("bakir-get -a [الرابط]  : تحميل مقطع صوتي فقط (MP3)");
    println!("bakir-get -f [الرابط]  : تحميل ملفات برامج، صور، و ISO بسرعة صاروخية");
    println!("bakir-get -h           : عرض هذه القائمة");
    println!("------------------------------------------");
    println!("📂 سيتم حفظ كافة التحميلات في مجلد Downloads");
}

fn check_dependencies() {
    // التأكد من وجود محركات التحميل في النظام
    let _ = Command::new("sudo").args(&["apt", "install", "-y", "libnotify-bin", "aria2", "ffmpeg"]).status();
}

fn send_notify(title: &str, msg: &str) {
    let _ = Command::new("notify-send").args(&["-i", "download", title, msg]).status();
}

fn download_media(args: &[String], mode: &str) {
    if args.len() < 3 { return; }
    let url = &args[2];
    let home = env::var("HOME").unwrap_or_else(|_| "/home/bakir".into());
    let download_path = format!("{}/Downloads", home);
    let _ = fs::create_dir_all(&download_path);

    send_notify("Bakir-Get", "🚀 جاري سحب الوسائط وتجاوز الحماية...");

    let mut cmd = Command::new("yt-dlp");
    cmd.current_dir(&download_path);
    cmd.args(&["--user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) Chrome/122.0.0.0 Safari/537.36"]);
    
    if mode == "video" {
        cmd.args(&["-f", "bestvideo[ext=mp4]+bestaudio[ext=m4a]/best[ext=mp4]/best", url]);
    } else {
        cmd.args(&["-x", "--audio-format", "mp3", url]);
    }

    if cmd.status().unwrap().success() {
        send_notify("Bakir-Get", "✅ اكتمل التحميل في مجلد Downloads");
    }
}

fn download_file(args: &[String]) {
    if args.len() < 3 { return; }
    let url = &args[2];
    let home = env::var("HOME").unwrap_or_else(|_| "/home/bakir".into());
    let download_path = format!("{}/Downloads", home);
    let _ = fs::create_dir_all(&download_path);

    send_notify("Bakir-Get", "⚡ جاري التحميل الصاروخي للملف...");

    // محرك aria2 مع إعدادات متوافقة مع كافة السيرفرات
    let status = Command::new("aria2c")
        .current_dir(&download_path)
        .args(&[
            "--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64)", 
            "--max-connection-per-server=5", 
            "--continue=true",
            "--check-certificate=false",
            url
        ])
        .status();

    if status.unwrap().success() {
        send_notify("Bakir-Get", "✅ تم التحميل بنجاح");
    }
}