/// Embed a Windows resource manifest with icon.
/// Pass `manifest: Some(xml_str)` for UAC or other manifest content.
/// This is a no-op on non-Windows platforms.
#[cfg(windows)]
pub fn embed(icon_path: &std::path::Path, manifest: Option<&str>) {
    use winapi::um::winnt;

    let mut res = winres::WindowsResource::new();
    res.set_icon(icon_path.to_str().unwrap())
        .set_language(winnt::MAKELANGID(
            winnt::LANG_ENGLISH,
            winnt::SUBLANG_ENGLISH_US,
        ));
    if let Some(manifest) = manifest {
        res.set_manifest(manifest);
    }
    res.compile().unwrap();
}

#[cfg(not(windows))]
pub fn embed(_icon_path: &std::path::Path, _manifest: Option<&str>) {}
