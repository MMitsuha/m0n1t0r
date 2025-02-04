fn build_upx() {
    let upx = cmake::build("../deps/upx").join("bin").join("upx");
    println!("cargo:rustc-env=UPX_EXECUTABLE={}", upx.display());
}

fn main() {
    build_upx();
}
