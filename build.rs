fn main() {
    cc::Build::new()
        .file("src/term_width.c")
        .compile("term_width");

    println!("cargo:rerun-if-changed=src/term_width.c");
}
