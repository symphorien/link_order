fn main() {
    println!("cargo:rerun-if-changed=b.c");
    println!("cargo:rerun-if-changed=a.c");

    // build the external dependency
    std::process::Command::new("cc")
        .arg("-shared")
        .arg("-o")
        .arg("libb.so")
        .arg("b.c")
        .spawn().expect("build error");

    // our crate needs a wrapper to link to libx
    let mut builder = cc::Build::new();
    builder
        .opt_level(2)
        .file("a.c");
    builder.compile("liba");

    // link to the dependent lib, before the wrapper
    println!("cargo:rustc-link-lib=b");
    println!("cargo:rustc-link-search=.");
    
}
