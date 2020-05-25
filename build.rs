fn main() {
    cc::Build::new()
        .file("src/drat-trim.c")
        .opt_level(2)
        .compile("drat-trim");
}
