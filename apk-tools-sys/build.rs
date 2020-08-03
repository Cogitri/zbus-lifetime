fn main() {
    if let Ok(lib_path) = std::env::var("APK_LIB_PATH") {
        println!("cargo:rustc-link-search=all={}", lib_path);
    } else {
        println!("cargo:rustc-link-search=all=/usr/lib/sdk/Cogitri/lib");
    }
    system_deps::Config::new().probe().unwrap();
}
