use std::path::PathBuf;

const PROJECT_LIST: [&str; 1] = ["m0n1t0r-cpp-windows-lib"];

fn xmake_build() -> Vec<PathBuf> {
    PROJECT_LIST
        .iter()
        .map(|x| xmake::build(format!("{}", x)))
        .collect::<Vec<PathBuf>>()
}

fn main() {
    for path in xmake_build() {
        println!("cargo:rustc-link-search=native={}", path.display());
    }

    PROJECT_LIST.iter().for_each(|x| {
        println!("cargo:rustc-link-lib={}", x);
    });
}
