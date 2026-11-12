use cc;

fn build_c_impl() {
    let mut build = cc::Build::new();

    #[cfg(target_os = "windows")]
    build.file("src/win10/IddController.c");

    build.flag_if_supported("-Wno-c++0x-extensions");
    build.flag_if_supported("-Wno-return-type-c-linkage");
    build.flag_if_supported("-Wno-invalid-offsetof");
    build.flag_if_supported("-Wno-unused-parameter");

    if build.get_compiler().is_like_msvc() {
        build.define("WIN32", "");
        // Fork deviation: -Z7 removed. With cc-rs's /Brepro, cl.exe writes the
        // command line into the object's debug record; on machines with a very
        // large environment (cargo env + long PATH) the record overflows and
        // cl fails with D8050. Debug info in the .lib is not needed for release.
        // build.flag("-Z7");
        build.flag("-GR-");
        // build.flag("-std:c++11");
    } else {
        build.flag("-fPIC");
        // build.flag("-std=c++11");
        // build.flag("-include");
        // build.flag(&confdefs_path.to_string_lossy());
    }

    #[cfg(target_os = "windows")]
    build.compile("win_virtual_display");

    #[cfg(target_os = "windows")]
    println!("cargo:rerun-if-changed=src/win10/IddController.c");
}

fn main() {
    build_c_impl();
}
