//! On Windows, puts the application icon and the version (`OPENOMSI_VERSION`, as the CI
//! passes it) into the executable. The version and commit the game shows are stamped by
//! omsi-platform's `build.rs`.

fn main() {
    println!("cargo:rerun-if-env-changed=OPENOMSI_VERSION");
    windows_icon();
    // (the executable exports the two switchable-graphics hints of main.rs, see there)
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows")
        && std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc")
    {
        for sym in [
            "NvOptimusEnablement",
            "AmdPowerXpressRequestHighPerformance",
        ] {
            println!("cargo:rustc-link-arg-bin=openomsi=/EXPORT:{sym},DATA");
        }
    }
}

fn windows_icon() {
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() != Ok("windows") {
        return;
    }
    println!("cargo:rerun-if-changed=../../assets/icons/app/openomsi.ico");
    let mut res = winresource::WindowsResource::new();
    res.set_icon("../../assets/icons/app/openomsi.ico")
        .set("ProductName", "openOMSI")
        .set("FileDescription", "openOMSI")
        .set(
            "ProductVersion",
            &std::env::var("OPENOMSI_VERSION").unwrap_or_default(),
        );
    if let Err(e) = res.compile() {
        println!("cargo:warning=no icon in the executable: {e}");
    }
}
