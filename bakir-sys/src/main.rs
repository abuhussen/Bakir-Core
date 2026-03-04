use std::process::Command;
use std::io::{self, Write};
use colored::*;
use notify_rust::Notification;

fn main() {
    println!("\n{}", "==================================================".cyan());
    println!("{}", "   📊 BAKIR-SYS v2.0 | مراقب النظام السيادي".bold().bright_white());
    println!("{}", "==================================================".cyan());

    // 1. المعالج
    let cpu = Command::new("sh").arg("-c").arg("grep 'model name' /proc/cpuinfo | head -1 | cut -d: -f2").output().unwrap();
    println!("💻 {}: {}", "المعالج".yellow(), String::from_utf8_lossy(&cpu.stdout).trim());

    // 2. الرام
    let mem_output = Command::new("free").arg("-m").output().unwrap();
    let mem_str = String::from_utf8_lossy(&mem_output.stdout);
    let lines: Vec<&str> = mem_str.lines().collect();
    let fields: Vec<&str> = lines[1].split_whitespace().collect();
    let total_mem: f32 = fields[1].parse().unwrap();
    let used_mem: f32 = fields[2].parse().unwrap();
    let mem_percent = (used_mem / total_mem) * 100.0;

    if mem_percent > 90.0 {
        Notification::new().summary("⚠️ تحذير الذاكرة").body("الرام تجاوزت 90%!").show().unwrap();
    }
    println!("🧠 {}: {:.1}% ({} MB / {} MB)", "الذاكرة".yellow(), mem_percent, used_mem, total_mem);

    // 3. الحرارة (فحص عام)
    let temp_raw = Command::new("sh").arg("-c").arg("sensors 2>/dev/null | grep 'Package id 0' | awk '{print $4}'").output().unwrap();
    let temp_str = String::from_utf8_lossy(&temp_raw.stdout).trim().to_string();
    println!("🌡️ {}: {}", "الحرارة".yellow(), if temp_str.is_empty() { "N/A".into() } else { temp_str.cyan().to_string() });

    // 4. وقت التشغيل
    let uptime = Command::new("uptime").arg("-p").output().unwrap();
    println!("⏱️ {}: {}", "مدة التشغيل".yellow(), String::from_utf8_lossy(&uptime.stdout).trim());

    println!("{}", "==================================================\n".cyan());
}
