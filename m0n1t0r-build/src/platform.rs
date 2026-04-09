/// Call this from build.rs to ensure a platform feature is set.
/// Panics with a helpful message if no platform feature (winnt, linux, macos, general) is detected.
pub fn ensure() {
    let has_platform = [
        std::env::var("CARGO_FEATURE_WINNT").is_ok(),
        std::env::var("CARGO_FEATURE_LINUX").is_ok(),
        std::env::var("CARGO_FEATURE_MACOS").is_ok(),
        std::env::var("CARGO_FEATURE_GENERAL").is_ok(),
    ];
    if !has_platform.iter().any(|&x| x) {
        panic!(
            "No target platform specified. Set one of: --features winnt, linux, macos, or general"
        );
    }
}
