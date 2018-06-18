use std::env;
extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .cpp_link_stdlib("c++")
        .warnings_into_errors(true)
        .warnings(false)
        .file("src/worker.cc")
        .compile("code");
}
