fn main() {
    println!("cargo::rerun-if-changed=native/checksum.c");
    cc::Build::new()
        .file("native/checksum.c")
        .warnings(true)
        .compile("appendix_checksum");
}
