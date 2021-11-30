
fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/markov.cpp")
        .flag_if_supported("-std=c++17")
        .compile("markov");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/markov.cpp");
    println!("cargo:rerun-if-changed=src/markov.h");
}
