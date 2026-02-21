use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let repo_raw = "https://raw.githubusercontent.com/abuhussen/Bakir-Core/main/remote-repo";
    let api_url = "https://api.github.com/repos/abuhussen/Bakir-Core/contents/remote-repo";

    if args.len() == 1 {
        println!("📡 رادار Bakir Linux | فحص المخزن السحابي...");
        println!("------------------------------------------");

        let output = Command::new("curl")
            .arg("-s")
            .arg("-L")
            .arg("-H")
            .arg("User-Agent: bakir-terminal")
            .arg(api_url)
            .output()
            .expect("فشل الاتصال");

        let body = String::from_utf8_lossy(&output.stdout);
        
        // طريقة أكثر ذكاءً لاستخراج الأسماء
        for line in body.split(',') {
            if line.contains("\"name\":") {
                let name = line.split(":").last().unwrap_or("").trim_matches(|c| c == '"' || c == ' ' || c == '}');
                if !name.is_empty() && name != "bakir-store" {
                    println!("📦 أداة متاحة: {}", name);
                }
            }
        }
        
        println!("------------------------------------------");
        println!("🚀 للتحميل: bakir-store -i [اسم_الأداة]");
        return;
    }

    if args[1] == "-i" && args.len() > 2 {
        let tool = &args[2];
        let target = format!("/usr/bin/{}", tool);
        let url = format!("{}/{}", repo_raw, tool);
        println!("📥 جاري سحب [{}]...", tool);
        let status = Command::new("sudo").args(&["wget", "-q", "--show-progress", "-O", &target, &url]).status().expect("فشل");
        if status.success() {
            Command::new("sudo").args(&["chmod", "+x", &target]).status().unwrap();
            println!("✅ تم التثبيت: [{}]", tool);
        }
    }
}