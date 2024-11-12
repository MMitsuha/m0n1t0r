use std::path::PathBuf;

#[allow(warnings)]
const PROJECT_LIST_WINDOWS: [&str; 1] = ["m0n1t0r-cpp-windows-lib"];

fn xmake_build() -> Vec<PathBuf> {
    #[allow(warnings)]
    let mut paths = Vec::new();

    #[cfg(feature = "windows")]
    paths.append(
        &mut PROJECT_LIST_WINDOWS
            .iter()
            .map(|x| xmake::build(format!("{}", x)))
            .collect::<Vec<PathBuf>>(),
    );

    paths
}

fn main() {
    for path in xmake_build() {
        println!("cargo:rustc-link-search=native={}", path.display());
    }

    #[cfg(feature = "windows")]
    PROJECT_LIST_WINDOWS.iter().for_each(|x| {
        println!("cargo:rustc-link-lib={}", x);
    });
}
