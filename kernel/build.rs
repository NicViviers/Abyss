use std::env;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn main() {
    let pico_sdk_path = PathBuf::from("/home/nicholas/.pico-sdk/sdk/2.2.0");
    let abyss_root = PathBuf::from("/home/nicholas/IdeaProjects/Abyss");
    let c_build_dir = abyss_root.join("build");
    let libraries = abyss_root.join("libraries");
    let examples = abyss_root.join("examples");

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        .use_core()
        .generate_inline_functions(true)
        .ctypes_prefix("cty")
        .clang_arg("--target=thumbv8m.main-none-eabihf")
        .clang_arg("-ffreestanding")
        .clang_arg("-DPICO_RP2350=1")
        .clang_arg("-DPICO_BOARD=\"pico2\"")
        .clang_arg("-DPICO_ON_DEVICE=1")
        .clang_arg(format!("-I{}", examples.display()))
        .clang_arg(format!("-I{}", examples.join("lvgl/lv_port").display()));

    // 1. SDK Discovery (Skips bazel, host, and rp2040 to avoid conflicts)
    for entry in WalkDir::new(&pico_sdk_path)
        .into_iter()
        .filter_entry(|e| {
            let name = e.file_name().to_str().unwrap_or("");
            name != "bazel" && name != "host" && name != "rp2040"
        })
        .filter_map(|e| e.ok())
        .filter(|e| e.file_name() == "include") 
    {
        builder = builder.clang_arg(format!("-I{}", entry.path().display()));
    }

    for entry in WalkDir::new(&libraries)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
    {
        let path = entry.path().display().to_string();
        println!("cargo:rerun-if-changed={}", path); // Rebuild if lib folders change
        builder = builder.clang_arg(format!("-I{}", path));
    }

    // 3. Generated Headers (Must exist for RP2350)
    let gen_path = c_build_dir.join("generated/pico_base");
    if gen_path.exists() {
        builder = builder.clang_arg(format!("-I{}", gen_path.display()));
    }

    // 4. Toolchain Headers (Using -isystem to prevent redefinition errors)
    let toolchain_base = "/home/nicholas/.pico-sdk/toolchain/14_2_Rel1";
    builder = builder
        .clang_arg(format!("-isystem{}/arm-none-eabi/include", toolchain_base))
        .clang_arg(format!("-isystem{}/lib/gcc/arm-none-eabi/14.2.1/include", toolchain_base));

    let bindings = builder.generate().expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs")).expect("Couldn't write!");
}