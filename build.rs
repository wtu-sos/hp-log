extern crate cc;

fn main() {
    cc::Build::new()
        .file("cpp/lib.cpp")
        .cpp(true)
        .shared_flag(true)
        .static_flag(true)
        .flag("-l./target/debug/")
        .include("./target/debug/")
        .out_dir("./target/debug/")
        .compile("hplogcpp");
}
