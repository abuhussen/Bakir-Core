use std::process::Command;
use std::io::{self, Write};

fn main() {
    if unsafe { libc::geteuid() } != 0 {
        println!("\x1b[1;31m❌ خطأ سيادي: يجب التشغيل بصلاحيات sudo\x1b[0m");
        return;
    }

    println!("\n\x1b[1;36m⚔️  BAKIR-KILLER v2.0 | قناص العمليات\x1b[0m");
    println!("--------------------------------------------------");
    println!("1. تطهير الزومبي (Z)");
    println!("2. قنص الذاكرة (>10%)");
    println!("3. إعدام برنامج باسمه");
    print!("\n🎯 اختر هدفك: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => {
            Command::new("sh").arg("-c").arg("ps -eo state,pid | grep '^Z' | awk '{print $2}' | xargs -r kill -9").status().unwrap();
            println!("✅ تم التطهير.");
        }
        "2" => {
            Command::new("sh").arg("-c").arg("ps -eo pmem,pid | awk '$1 > 10.0 {print $2}' | xargs -r kill -9").status().unwrap();
            println!("✅ تم قنص المستهلكين.");
        }
        "3" => {
            print!("✍️ اسم البرنامج: ");
            io::stdout().flush().unwrap();
            let mut name = String::new();
            io::stdin().read_line(&mut name).unwrap();
            Command::new("pkill").arg("-9").arg(name.trim()).status().unwrap();
            println!("🎯 تم الإعدام.");
        }
        _ => println!("❌ اختيار خاطئ."),
    }
}
