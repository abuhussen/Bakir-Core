use std::process::Command;
use std::env;
use colored::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 { display_help(); return; }

    let mode = &args[1];
    let url = &args[2];

    println!("{}", "==================================================".cyan());
    println!("{}", "🚀 Bakir-Git v1.2 | النسخة السيادية الذكية".bold().bright_red());
    println!("{}", "==================================================".cyan());

    match mode.as_str() {
        "-v" => download_video(url),
        "-a" => download_audio(url),
        "-s" => download_general(url),
        _ => display_help(),
    }
}

fn download_video(url: &str) {
    println!("🎬 جاري سحب الفيديو (فردي فقط)...");
    let status = Command::new("yt-dlp")
    .args(&[
        "--no-playlist", // 💡 هذا الأمر السحري لمنع تحميل القوائم كاملة
        "-f", "bv+ba/b",
        "--embed-subs",
        "-o", "%(title)s.%(ext)s",
          url
    ])
    .status();
    check_status(status, "اكتمل تحميل الفيديو بنجاح!");
}

fn download_audio(url: &str) {
    println!("🎵 جاري استخراج الصوت (فردي فقط)...");
    let status = Command::new("yt-dlp")
    .args(&[
        "--no-playlist", // 💡 تحميل الصوت للفيديو الحالي فقط
        "-x",
        "--audio-format", "mp3",
        "--audio-quality", "0",
        "-o", "%(title)s.%(ext)s",
          url
    ])
    .status();
    check_status(status, "اكتمل تحميل الملف الصوتي!");
}

fn download_general(url: &str) {
    println!("📦 جاري التحميل المباشر للملف...");
    let status = Command::new("curl")
    .args(&["-O", "-L", "-C", "-", url])
    .status();
    check_status(status, "تم تحميل الملف بنجاح!");
}

fn check_status(status: std::io::Result<std::process::ExitStatus>, msg: &str) {
    match status {
        Ok(s) if s.success() => println!("✅ {}", msg.green().bold()),
        _ => println!("❌ {}", "حدث خطأ. تأكد من تحديث المحرك yt-dlp.".red()),
    }
}

fn display_help() {
    println!("{}", "\n⚔️ Bakir-Git | دليل الاستخدام الذكي".bold().bright_cyan());
    println!(" • bakir-git -v [URL]  | تحميل فيديو واحد (يتجاهل القوائم)");
    println!(" • bakir-git -a [URL]  | تحميل صوت واحد MP3");
    println!(" • bakir-git -s [URL]  | تحميل ملفات متنوعة\n");
}
