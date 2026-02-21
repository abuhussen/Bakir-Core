use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // الرابط الخام للتحميل
    let repo_raw = "https://raw.githubusercontent.com/abuhussen/Bakir-Core/main/remote-repo";
    // رابط الـ API لجلب قائمة الملفات ديناميكياً
    let api_url = "https://api.github.com/repos/abuhussen/Bakir-Core/contents/remote-repo";

    if args.len() == 1 {
        println!("📡 رادار Bakir Linux | فحص المخزن السحابي...");
        println!("------------------------------------------");

        // استخدام curl لجلب قائمة الملفات من GitHub API
        let output = Command::new("curl")
            .arg("-s")
            .arg("-H")
            .arg("Accept: application/vnd.github.v3+json")
            .arg(api_url)
            .output()
            .expect("فشل الاتصال بالعرين");

        let body = String::from_utf8_lossy(&output.stdout);

        // تحليل بسيط للنص لجلب الأسماء فقط (بدون مكتبات خارجية لضمان السرعة)
        if body.contains("\"name\":") {
            for part in body.split("\"name\":\"") {
                if let Some(name) = part.split("\"").next() {
                    // استثناء الملفات التي لا نريد عرضها كبرامج
                    if !name.is_empty() && !name.contains("{") && name != "bakir-store" {
                        println!("📦 أداة متاحة: {}", name);
                    }
                }
            }
        }
        
        println!("------------------------------------------");
        println!("🚀 للتحميل والتثبيت: bakir-store -i [اسم_الأداة]");
        return;
    }

    if args[1] == "-i" && args.len() > 2 {
        let tool = &args[2];
        let target = format!("/usr/bin/{}", tool);
        let url = format!("{}/{}", repo_raw, tool);

        println!("📥 جاري سحب [{}] من السحاب إلى النظام...", tool);

        let status = Command::new("sudo")
            .args(&["wget", "-q", "--show-progress", "-O", &target, &url])
            .status()
            .expect("فشل التحميل");

        if status.success() {
            Command::new("sudo")
                .args(&["chmod", "+x", &target])
                .status()
                .unwrap();
            println!("✅ تم التثبيت بنجاح! يمكنك الآن كتابة [{}] في الترمينال.", tool);
        }
    }
}