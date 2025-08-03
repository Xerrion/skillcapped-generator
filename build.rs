use std::env;

fn main() {
    // Only build resources on Windows
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winresource::WindowsResource::new();
        // Set executable metadata
        res.set_version_info(winresource::VersionInfo::PRODUCTVERSION, 0x0001000000000000)
            .set_version_info(winresource::VersionInfo::FILEVERSION, 0x0001000000000000)
            .set("CompanyName", "Xerrion")
            .set("FileDescription", "SkillCapped Unlock Code Generator")
            .set("InternalName", "skillcapped-generator")
            .set("LegalCopyright", "© 2025 Xerrion. All rights reserved.")
            .set("OriginalFilename", "skillcapped-generator.exe")
            .set("ProductName", "SkillCapped Generator")
            .set("ProductVersion", "1.0.0");

        // Compile the resource
        if let Err(e) = res.compile() {
            // Don't fail the build if icon is missing, just print a warning
            if !e.to_string().contains("icon.ico") {
                panic!("Failed to compile resources: {}", e);
            } else {
                println!("cargo:warning=Icon file not found, building without icon");
            }
        }
    }

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=icon.ico");
}
