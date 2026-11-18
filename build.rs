use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=ASSIMP_LIB_DIR");
    println!("cargo:rerun-if-env-changed=ASSIMP_LIB_NAME");
    println!("cargo:rerun-if-env-changed=ASSIMP_STATIC");

    if let Some(dir) = env::var_os("ASSIMP_LIB_DIR") {
        println!("cargo:rustc-link-search=native={}", dir.to_string_lossy());
    }

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();

    // Assimp's CMake build names the MSVC library after the toolset
    // (`assimp-vc145-mt` for the official 6.0.6 binaries). MinGW and Unix
    // builds use `assimp`.
    let lib = env::var("ASSIMP_LIB_NAME").unwrap_or_else(|_| {
        if target_os == "windows" && target_env == "msvc" {
            "assimp-vc145-mt".to_owned()
        } else {
            "assimp".to_owned()
        }
    });

    let kind = if env::var_os("ASSIMP_STATIC").is_some() {
        "static"
    } else {
        "dylib"
    };
    println!("cargo:rustc-link-lib={kind}={lib}");
}
