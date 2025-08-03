use std::env;

fn main() {
    // Only build resources on Windows
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        println!("cargo:info=Building Windows resources with embed-resource...");

        if std::path::Path::new("app.rc").exists() {
            // Use manifest_required() since we have a manifest for Windows compatibility
            embed_resource::compile("app.rc", embed_resource::NONE)
                .manifest_required()
                .unwrap();
            println!("cargo:info=Successfully compiled resources with embed-resource");
        } else {
            println!("cargo:warning=app.rc file not found, building without resources");
        }
    } else {
        println!("cargo:info=Not building on Windows, skipping resource compilation");
    }

    // Rebuild triggers - according to embed-resource docs, these are needed
    // since the crate doesn't generate cargo:rerun-if-changed annotations automatically
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=app.rc");
    println!("cargo:rerun-if-changed=skillcapped-generator.exe.manifest");
    println!("cargo:rerun-if-changed=icon.ico");
}
