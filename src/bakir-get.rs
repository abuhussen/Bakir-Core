use std::process::{Command, Stdio};
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.contains(&"-h".to_string()) {
        display_help();
        return;
    }

    // فحص المتطلبات قبل البدء
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
    println!("bakir-get -v [الرابط]  : تحميل فيديو بأعلى جودة (MP4)");
    println!("bakir-get -a [الرابط]  : تحميل مقطع صوتي فقط (MP3)");
    println!("bakir-get -f [الرابط]  : تحميل ملف مباشر (برامج، صور، ISO) بسرعة صاروخية");
    println!("bakir-get -h           : عرض هذه القائمة التعليمية");
    println!("------------------------------------------");
    println!("ملاحظة: يتم حفظ التحميلات تلقائياً في مجلد Downloads.");
}

fn send_notify(title: &str, msg: &str) {
    let _ = Command::new("sudo")
        .args(&["-u", "bakir", "DISPLAY=:0", "DBUS_SESSION_BUS_ADDRESS=unix:path=/run/user/1000/bus", 
                "notify-send", title, msg, "-i", "download"])
        .status();
}

fn check_dependencies() {
    let deps = ["yt-dlp", "aria2c", "ffmpeg"];
    for dep in deps.iter() {
        if Command::new(dep).arg("--version").stdout(Stdio::null()).stderr(Stdio::null()).status().is_err() {
            println!("⚠️ الحزمة {} مفقودة، جاري تجهيزها...", dep);
            let _ = Command::new("sudo").args(&["apt", "install", "-y", dep]).status();
        }
    }
}

fn download_media(args: &[String], mode: &str) {
    if args.len() < 3 {
        println!("❌ يرجى إدخال الرابط.");
        return;
    }
    let url = &args[2];
    let download_dir = format!("{}/Downloads", env::var("HOME").unwrap_or_else(|_| "/home/bakir".into()));
    
    send_notify("📥 جاري التحميل", "بدأ حصن باكير بسحب الوسائط المطلوبة...");

    let mut cmd = Command::new("yt-dlp");
    cmd.current_dir(&download_dir);

    if mode == "video" {
        cmd.args(&["-f", "bestvideo+bestaudio/best", "--merge-output-format", "mp4", url]);
    } else {
        cmd.args(&["-x", "--audio-format", "mp3", url]);
    }

    let status = cmd.status();

    if status.is_ok() && status.unwrap().success() {
        send_notify("✅ اكتمل التحميل", "تم حفظ الملف في مجلد Downloads بنجاح.");
        println!("✨ تم التحميل بنجاح في: {}", download_dir);
    } else {
        println!("❌ فشل التحميل. تأكد من الرابط أو اتصال الإنترنت.");
    }
}

fn download_file(args: &[String]) {
    if args.len() < 3 {
        println!("❌ يرجى إدخال رابط الملف.");
        return;
    }
    let url = &args[2];
    let download_dir = format!("{}/Downloads", env::var("HOME").unwrap_or_else(|_| "/home/bakir".into()));

    send_notify("🚀 تحميل صاروخي", "جاري تحميل الملف باستخدام محرك aria2...");

    let status = Command::new("aria2c")
        .args(&["-d", &download_dir, "-x", "16", "-s", "16", url])
        .status();

    if status.is_ok() && status.unwrap().success() {
        send_notify("✅ اكتمل الملف", "تم تحميل الملف بنجاح وبأقصى سرعة.");
    } else {
        println!("❌ فشل تحميل الملف.");
    }
}