use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // الرابط السحابي المباشر لمستودعك (Raw)
    let repo_raw = "https://raw.githubusercontent.com/abuhussen/Bakir-Core/main/remote-repo";

    if args.len() < 2 {
        println!("\n📡 رادار Bakir Linux | المخزن السحابي");
        println!("-------------------------------------------");
        println!("💡 للاستخدام: bakir-store -i [اسم_الأداة]");
        return;
    }

    if args[1] == "-i" && args.len() > 2 {
        let tool = &args[2];
        println!("📥 جاري جلب الأداة [{}] من العرين السحابي...", tool);
        
        let target = format!("/usr/bin/{}", tool);
        let url = format!("{}/{}", repo_raw, tool);

        let status = Command::new("sudo")
            .args(&["wget", "-q", "--show-progress", "-O", &target, &url])
            .status()
            .expect("فشل الاتصال");

        if status.success() {
            Command::new("sudo").args(&["chmod", "+x", &target]).status().ok();
            println!("✅ تم التثبيت بنجاح! الأداة [{}] جاهزة الآن.", tool);
        } else {
            println!("❌ خطأ: لم أجد الأداة في المستودع.");
        }
    }
}