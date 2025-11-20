extern crate miette;

fn main() -> miette::Result<()> {
    println!("cargo:rerun-if-changed=src/cpp/mod.rs");

    cxx_build::bridge("src/cpp/mod.rs")
        .file("src/cpp/example.cpp")
        .include("src/cpp")
        .std("c++17")
        .compile("cpp");

    // uncommenting the following line makes no changes to any result.
    // println!("cargo:rustc-link-lib=static=cpp");

    Ok(())
}
