# Build Scripts Refactor Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Restructure the build system so m0n1t0r-build is the central build toolkit, build.rs files are thin orchestrators, and all bugs/duplication/dead code are eliminated.

**Architecture:** Move platform validation and winres embedding into m0n1t0r-build as reusable modules. Fix cert/config naming inconsistency. Clean up xtask (deduplicate cert DN, add --force, remove dead features). Remove empty m0n1t0r-macro crate.

**Tech Stack:** Rust build scripts, cargo-emit, winres, cxx-build, xmake-rs, rcgen, vergen, clap

---

### Task 1: Remove m0n1t0r-macro crate

**Files:**
- Delete: `m0n1t0r-macro/` (entire directory)
- Modify: `Cargo.toml:3` (workspace members)

- [ ] **Step 1: Remove from workspace members**

In `Cargo.toml`, change:

```toml
[workspace]
resolver = "2"
members = [
    "m0n1t0r-client",
    "m0n1t0r-server",
    "m0n1t0r-common",
    "xtask", "m0n1t0r-build",
]
```

(No change needed — m0n1t0r-macro is already not listed in workspace members. It was a standalone crate.)

- [ ] **Step 2: Delete the crate directory**

```bash
rm -rf m0n1t0r-macro/
```

- [ ] **Step 3: Verify workspace builds**

```bash
cargo check --workspace --features winnt 2>&1 | tail -5
```

Expected: No errors mentioning m0n1t0r-macro.

- [ ] **Step 4: Commit**

```bash
git add -A && git commit -m "chore: remove empty m0n1t0r-macro crate"
```

---

### Task 2: Fix cert module naming and bug

**Files:**
- Modify: `m0n1t0r-build/src/cert.rs`

The current `check()` returns `true` when certs are MISSING (opposite of `config::check()`). The `ensure()` function has a logic bug — it panics inside `if check()` but then falls through to emit `rerun-if-changed` for potentially missing files.

- [ ] **Step 1: Rewrite cert.rs**

Replace the entire content of `m0n1t0r-build/src/cert.rs` with:

```rust
use crate::config;
use std::path::PathBuf;

pub fn paths() -> [PathBuf; 3] {
    let root = std::path::Path::new(env!("CARGO_WORKSPACE_DIR"));
    let config = config::read();
    [
        root.join(&config.tls.ca),
        root.join(&config.tls.cert),
        root.join(&config.tls.key),
    ]
}

pub fn exists() -> bool {
    paths().iter().all(|cert| cert.exists())
}

pub fn ensure() {
    for cert in &paths() {
        cargo_emit::rerun_if_changed!(cert.display());
    }
    if !exists() {
        let missing: Vec<_> = paths()
            .into_iter()
            .filter(|c| !c.exists())
            .collect();
        panic!(
            "Missing certificate(s): {}. Please run `cargo xtask -c` to generate.",
            missing
                .iter()
                .map(|p| p.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
}
```

Key changes:
- `path()` renamed to `paths()` (it returns an array)
- `check()` renamed to `exists()` — returns `true` when certs DO exist (consistent with config)
- `ensure()` bug fixed — always emits `rerun-if-changed`, only panics if missing

- [ ] **Step 2: Update callers of cert::check and cert::path**

In `m0n1t0r-common/build.rs` — no changes needed (only calls `cert::ensure()`).

In `xtask/src/main.rs`, line 181, change:

```rust
        if !build_cert::check() {
```

to:

```rust
        if build_cert::exists() {
```

(Old `check()` returned `true` when missing, new `exists()` returns `true` when present — so the guard logic inverts.)

- [ ] **Step 3: Verify it compiles**

```bash
cargo check -p m0n1t0r-build && cargo check -p xtask
```

Expected: Clean compilation.

- [ ] **Step 4: Commit**

```bash
git add m0n1t0r-build/src/cert.rs xtask/src/main.rs && git commit -m "fix: rename cert::check to cert::exists, fix ensure() logic bug"
```

---

### Task 3: Fix config module naming

**Files:**
- Modify: `m0n1t0r-build/src/config.rs`
- Modify: `xtask/src/main.rs`

Rename `check()` to `exists()` for consistency with the cert module.

- [ ] **Step 1: Rename check to exists in config.rs**

In `m0n1t0r-build/src/config.rs`, rename the function at line 100:

```rust
pub fn exists() -> bool {
```

And update `ensure()` at line 92 to call `exists()`:

```rust
    if !exists() {
```

- [ ] **Step 2: Update xtask caller**

In `xtask/src/main.rs`, line 173, change:

```rust
        if build_config::check() {
```

to:

```rust
        if build_config::exists() {
```

- [ ] **Step 3: Verify it compiles**

```bash
cargo check -p m0n1t0r-build && cargo check -p xtask
```

- [ ] **Step 4: Commit**

```bash
git add m0n1t0r-build/src/config.rs xtask/src/main.rs && git commit -m "refactor: rename config::check to config::exists for consistency"
```

---

### Task 4: Add platform module to m0n1t0r-build

**Files:**
- Create: `m0n1t0r-build/src/platform.rs`
- Modify: `m0n1t0r-build/src/lib.rs`

This replaces the duplicated `compile_error!` blocks in client and server build.rs.

- [ ] **Step 1: Create platform.rs**

Create `m0n1t0r-build/src/platform.rs`:

```rust
/// Call this from build.rs to emit a compile_error if no platform feature is set.
/// Must be paired with the crate having features: general, winnt, linux, macos.
pub fn ensure() {
    let dominated_by_cargo_features = [
        std::env::var("CARGO_FEATURE_WINNT").is_ok(),
        std::env::var("CARGO_FEATURE_LINUX").is_ok(),
        std::env::var("CARGO_FEATURE_MACOS").is_ok(),
        std::env::var("CARGO_FEATURE_GENERAL").is_ok(),
    ];
    if !dominated_by_cargo_features.iter().any(|&x| x) {
        panic!(
            "No target platform specified. Set one of: --features winnt, linux, macos, or general"
        );
    }
}
```

Note: We use a build.rs runtime check instead of `compile_error!` because `compile_error!` is a source-level attribute that can't be shared from a library. A `panic!` in build.rs achieves the same effect — build fails with a clear message.

- [ ] **Step 2: Add module to lib.rs**

In `m0n1t0r-build/src/lib.rs`, add:

```rust
pub mod cert;
pub mod config;
pub mod dep;
pub mod platform;
pub mod version;
```

- [ ] **Step 3: Verify it compiles**

```bash
cargo check -p m0n1t0r-build
```

- [ ] **Step 4: Commit**

```bash
git add m0n1t0r-build/src/platform.rs m0n1t0r-build/src/lib.rs && git commit -m "feat: add platform validation module to m0n1t0r-build"
```

---

### Task 5: Add winres module to m0n1t0r-build

**Files:**
- Create: `m0n1t0r-build/src/winres.rs`
- Modify: `m0n1t0r-build/src/lib.rs`
- Modify: `m0n1t0r-build/Cargo.toml`

This replaces the duplicated `add_manifest_windows()` in client and server build.rs.

- [ ] **Step 1: Add winres and winapi as optional build deps**

In `m0n1t0r-build/Cargo.toml`, add:

```toml
[dependencies]
cargo-emit = "0.2.1"
vergen = { version = "9.0.6", features = ["build", "cargo", "rustc", "si"] }
execute = "0.2.13"
regex = "1.12.2"
strip-ansi-escapes = "0.2.1"
toml = "0.8"
serde = { version = "1", features = ["derive"] }

[target.'cfg(windows)'.dependencies]
winres = { version = "0.1.12", optional = true }
winapi = { version = "0.3.9", features = ["winbase"], optional = true }

[features]
default = []
winres = ["dep:winres", "dep:winapi"]
```

- [ ] **Step 2: Create winres.rs**

Create `m0n1t0r-build/src/winres.rs`:

```rust
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
```

- [ ] **Step 3: Add module to lib.rs**

In `m0n1t0r-build/src/lib.rs`:

```rust
pub mod cert;
pub mod config;
pub mod dep;
pub mod platform;
pub mod version;
#[cfg(feature = "winres")]
pub mod winres;
```

- [ ] **Step 4: Verify it compiles**

```bash
cargo check -p m0n1t0r-build --features winres
```

- [ ] **Step 5: Commit**

```bash
git add m0n1t0r-build/src/winres.rs m0n1t0r-build/src/lib.rs m0n1t0r-build/Cargo.toml && git commit -m "feat: add winres module to m0n1t0r-build"
```

---

### Task 6: Simplify dep.rs — remove unused deps

**Files:**
- Modify: `m0n1t0r-build/src/dep.rs`
- Modify: `m0n1t0r-build/Cargo.toml`

Remove `nom` (unused) and `execute` (replace with `std::process::Command`).

- [ ] **Step 1: Rewrite dep.rs to use std::process::Command**

Replace the entire content of `m0n1t0r-build/src/dep.rs` with:

```rust
use regex::Regex;
use std::process::Command;

pub fn check_xmake() {
    Command::new("xmake")
        .arg("--help")
        .output()
        .expect("No xmake found. Please install xmake.");
}

pub fn check_xrepo() {
    let output = Command::new("xrepo")
        .arg("--version")
        .output()
        .expect("Failed to run xrepo. Please install xrepo.");
    let stdout = String::from_utf8_lossy(&output.stdout);
    if !stdout.contains("xRepo") {
        panic!("No xrepo found. Please install xrepo. Output: {stdout}.");
    }
}

fn extract_paths(regex: &Regex, data: &str) -> Vec<String> {
    let mut paths = Vec::new();
    if let Some(captures) = regex.captures(data)
        && let Some(content) = captures.get(1)
    {
        for quoted_path in content.as_str().split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
            paths.push(quoted_path.trim_matches('"').to_string());
        }
    }
    paths
}

pub fn xrepo_fetch(dep: &str) -> (Vec<String>, Vec<String>) {
    let output = Command::new("xrepo")
        .arg("fetch")
        .arg(dep)
        .output()
        .unwrap_or_else(|_| panic!("Failed to fetch dependency: {dep}."));
    let stdout =
        String::from_utf8(strip_ansi_escapes::strip(&output.stdout))
            .expect("Failed to convert xrepo output to string.");
    let links = extract_paths(
        &Regex::new(r#"[^sys]links\s*=\s*\{\s*((?:"[^"]*",?\s*)*)\s*\}"#).unwrap(),
        &stdout,
    );
    let link_dirs = extract_paths(
        &Regex::new(r#"linkdirs\s*=\s*\{\s*((?:"[^"]*",?\s*)*)\s*\}"#).unwrap(),
        &stdout,
    );
    (links, link_dirs)
}
```

- [ ] **Step 2: Remove nom and execute from Cargo.toml**

In `m0n1t0r-build/Cargo.toml`, remove these two lines:

```toml
nom = "8.0.0"
execute = "0.2.13"
```

The `[dependencies]` section should become:

```toml
[dependencies]
cargo-emit = "0.2.1"
vergen = { version = "9.0.6", features = ["build", "cargo", "rustc", "si"] }
regex = "1.12.2"
strip-ansi-escapes = "0.2.1"
toml = "0.8"
serde = { version = "1", features = ["derive"] }
```

- [ ] **Step 3: Verify it compiles**

```bash
cargo check -p m0n1t0r-build
```

- [ ] **Step 4: Commit**

```bash
git add m0n1t0r-build/src/dep.rs m0n1t0r-build/Cargo.toml && git commit -m "refactor: remove unused nom/execute deps, use std::process::Command"
```

---

### Task 7: Refactor client build.rs to use m0n1t0r-build modules

**Files:**
- Modify: `m0n1t0r-client/build.rs`
- Modify: `m0n1t0r-client/Cargo.toml`

Replace duplicated platform validation and winres code with m0n1t0r-build calls.

- [ ] **Step 1: Update client build-dependencies**

In `m0n1t0r-client/Cargo.toml`, replace the build-dependencies sections:

```toml
[build-dependencies]
xmake = { git = "https://github.com/MMitsuha/xmake-rs.git" }
cxx-build = "1.0.187"
m0n1t0r-build = { path = "../m0n1t0r-build", features = ["winres"] }
cargo-emit = "0.2.1"

[target.'cfg(windows)'.build-dependencies]
winapi = { version = "0.3.9", features = ["winbase"] }
```

Remove the `winres` line from `[target.'cfg(windows)'.build-dependencies]` (it's now pulled in transitively via m0n1t0r-build's `winres` feature). Keep `winapi` because the client still needs `winapi` as a runtime dependency (it's in `[target.'cfg(windows)'.dependencies]` too) — but actually `winapi` for build was only used by the old `add_manifest_windows()`. So remove it from build-dependencies entirely:

```toml
[build-dependencies]
xmake = { git = "https://github.com/MMitsuha/xmake-rs.git" }
cxx-build = "1.0.187"
m0n1t0r-build = { path = "../m0n1t0r-build", features = ["winres"] }
cargo-emit = "0.2.1"
```

Delete the entire `[target.'cfg(windows)'.build-dependencies]` section.

- [ ] **Step 2: Rewrite client build.rs**

Replace the entire content of `m0n1t0r-client/build.rs` with:

```rust
use m0n1t0r_build::{cert, config, dep};
use std::path::Path;

fn bridge_build(bridges: &[&str]) {
    bridges.iter().for_each(|x| {
        let _ = cxx_build::bridge(x);
    });
}

fn xmake_build(project: &str) {
    let path = Path::new(env!("CARGO_WORKSPACE_DIR"))
        .join("m0n1t0r-client")
        .join(project);
    xmake::build(&path);
    cargo_emit::rerun_if_changed!(path.display());
}

fn main() {
    m0n1t0r_build::platform::ensure();
    dep::check_xmake();
    dep::check_xrepo();

    config::ensure();
    cert::ensure();

    let cfg = config::read();
    cargo_emit::rustc_env!("M0N1T0R_DOMAIN", "{}", cfg.cert.domain);
    cargo_emit::rustc_env!("M0N1T0R_CONN_PORT", "{}", cfg.conn.addr.port());
    cargo_emit::rustc_env!(
        "M0N1T0R_CA",
        "{}",
        Path::new(env!("CARGO_WORKSPACE_DIR"))
            .join(&cfg.tls.ca)
            .display()
    );

    bridge_build(&["src/init.rs"]);
    #[cfg(feature = "winnt")]
    bridge_build(&[
        "src/client/windows/autorun.rs",
        "src/client/windows/process.rs",
        "src/client/windows/charset.rs",
        "src/client/windows/blind.rs",
    ]);

    xmake_build("m0n1t0r-cpp-general-lib");
    #[cfg(feature = "winnt")]
    xmake_build("m0n1t0r-cpp-windows-lib");

    #[cfg(feature = "winnt")]
    m0n1t0r_build::winres::embed(
        &Path::new(env!("CARGO_WORKSPACE_DIR")).join("resource/mc.ico"),
        Some(
            r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
<trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
        <requestedPrivileges>
            <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
        </requestedPrivileges>
    </security>
</trustInfo>
</assembly>
"#,
        ),
    );
    #[cfg(all(feature = "winnt", not(feature = "winnt-uac")))]
    m0n1t0r_build::winres::embed(
        &Path::new(env!("CARGO_WORKSPACE_DIR")).join("resource/mc.ico"),
        None,
    );
}
```

Wait — the original code only sets the UAC manifest when `winnt-uac` is set, but always embeds the icon when `winnt` is set. Let me correct:

```rust
    #[cfg(feature = "winnt")]
    {
        let icon = Path::new(env!("CARGO_WORKSPACE_DIR")).join("resource/mc.ico");
        #[cfg(feature = "winnt-uac")]
        let manifest = Some(
            r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
<trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
        <requestedPrivileges>
            <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
        </requestedPrivileges>
    </security>
</trustInfo>
</assembly>
"#,
        );
        #[cfg(not(feature = "winnt-uac"))]
        let manifest = None;
        m0n1t0r_build::winres::embed(&icon, manifest);
    }
```

Full final `m0n1t0r-client/build.rs`:

```rust
use m0n1t0r_build::{cert, config, dep};
use std::path::Path;

fn bridge_build(bridges: &[&str]) {
    bridges.iter().for_each(|x| {
        let _ = cxx_build::bridge(x);
    });
}

fn xmake_build(project: &str) {
    let path = Path::new(env!("CARGO_WORKSPACE_DIR"))
        .join("m0n1t0r-client")
        .join(project);
    xmake::build(&path);
    cargo_emit::rerun_if_changed!(path.display());
}

fn main() {
    m0n1t0r_build::platform::ensure();
    dep::check_xmake();
    dep::check_xrepo();

    config::ensure();
    cert::ensure();

    let cfg = config::read();
    cargo_emit::rustc_env!("M0N1T0R_DOMAIN", "{}", cfg.cert.domain);
    cargo_emit::rustc_env!("M0N1T0R_CONN_PORT", "{}", cfg.conn.addr.port());
    cargo_emit::rustc_env!(
        "M0N1T0R_CA",
        "{}",
        Path::new(env!("CARGO_WORKSPACE_DIR"))
            .join(&cfg.tls.ca)
            .display()
    );

    bridge_build(&["src/init.rs"]);
    #[cfg(feature = "winnt")]
    bridge_build(&[
        "src/client/windows/autorun.rs",
        "src/client/windows/process.rs",
        "src/client/windows/charset.rs",
        "src/client/windows/blind.rs",
    ]);

    xmake_build("m0n1t0r-cpp-general-lib");
    #[cfg(feature = "winnt")]
    xmake_build("m0n1t0r-cpp-windows-lib");

    #[cfg(feature = "winnt")]
    {
        let icon = Path::new(env!("CARGO_WORKSPACE_DIR")).join("resource/mc.ico");
        #[cfg(feature = "winnt-uac")]
        let manifest = Some(r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
<trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
        <requestedPrivileges>
            <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
        </requestedPrivileges>
    </security>
</trustInfo>
</assembly>
"#);
        #[cfg(not(feature = "winnt-uac"))]
        let manifest = None;
        m0n1t0r_build::winres::embed(&icon, manifest);
    }
}
```

- [ ] **Step 3: Verify it compiles**

```bash
cargo check -p m0n1t0r-client --features winnt
```

Expected: Clean compilation. (Full build requires xmake + config.toml + certs, but `check` should pass for Rust code.)

- [ ] **Step 4: Commit**

```bash
git add m0n1t0r-client/build.rs m0n1t0r-client/Cargo.toml && git commit -m "refactor: client build.rs uses m0n1t0r-build platform/winres modules"
```

---

### Task 8: Refactor server build.rs to use m0n1t0r-build modules

**Files:**
- Modify: `m0n1t0r-server/build.rs`
- Modify: `m0n1t0r-server/Cargo.toml`

- [ ] **Step 1: Update server build-dependencies**

In `m0n1t0r-server/Cargo.toml`, replace the `[target.'cfg(windows)'.build-dependencies]` section with:

```toml
[build-dependencies]
m0n1t0r-build = { path = "../m0n1t0r-build", features = ["winres"] }
```

Delete the existing `[target.'cfg(windows)'.build-dependencies]` section (lines 76-78).

- [ ] **Step 2: Rewrite server build.rs**

Replace the entire content of `m0n1t0r-server/build.rs` with:

```rust
use std::path::Path;

fn main() {
    m0n1t0r_build::platform::ensure();

    #[cfg(feature = "winnt")]
    m0n1t0r_build::winres::embed(
        &Path::new(env!("CARGO_WORKSPACE_DIR")).join("resource/ms.ico"),
        None,
    );
}
```

- [ ] **Step 3: Verify it compiles**

```bash
cargo check -p m0n1t0r-server --features winnt
```

- [ ] **Step 4: Commit**

```bash
git add m0n1t0r-server/build.rs m0n1t0r-server/Cargo.toml && git commit -m "refactor: server build.rs uses m0n1t0r-build platform/winres modules"
```

---

### Task 9: Clean up xtask — remove dead features, add --force, deduplicate cert DN

**Files:**
- Modify: `xtask/Cargo.toml`
- Modify: `xtask/src/main.rs`

- [ ] **Step 1: Remove unused features from xtask/Cargo.toml**

In `xtask/Cargo.toml`, delete lines 11-14:

```toml
[features]
general = []
winnt = []
linux = []
macos = []
```

- [ ] **Step 2: Rewrite xtask/src/main.rs**

Replace the entire content of `xtask/src/main.rs` with:

```rust
use anyhow::{Context, Result};
use clap::Parser;
use dialoguer::{Confirm, Input};
use flexi_logger::Logger;
use log::{info, warn};
use m0n1t0r_build::{
    cert as build_cert, config as build_config,
    config::{ApiConfig, CertConfig, ConnConfig, FileConfig, GeneralConfig, TlsConfig},
};
use rcgen::{CertificateParams, DnType, IsCa, KeyPair};
use std::fs;
use time::{Duration, OffsetDateTime};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Arguments {
    #[arg(short, long, help = "Generate TLS certificates")]
    cert: bool,
    #[arg(short, long, help = "Interactive config.toml generator")]
    init: bool,
    #[arg(short, long, help = "Force overwrite existing files")]
    force: bool,
}

fn populate_dn(params: &mut CertificateParams, cert: &CertConfig) {
    params
        .distinguished_name
        .push(DnType::CountryName, &cert.country);
    params
        .distinguished_name
        .push(DnType::StateOrProvinceName, &cert.state);
    params
        .distinguished_name
        .push(DnType::LocalityName, &cert.locality);
    params
        .distinguished_name
        .push(DnType::OrganizationName, &cert.org);
    params
        .distinguished_name
        .push(DnType::OrganizationalUnitName, &cert.unit);
    params
        .distinguished_name
        .push(DnType::CommonName, format!("{}.", &cert.domain));
}

fn generate_certs() -> Result<()> {
    let config = build_config::read();
    for path in [&config.tls.ca, &config.tls.cert, &config.tls.key] {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
    }

    let now = OffsetDateTime::now_utc();
    let not_after = now + Duration::days(3650);

    // Generate CA
    let mut ca_params = CertificateParams::new(Vec::<String>::new())?;
    ca_params.is_ca = IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
    ca_params.not_before = now;
    ca_params.not_after = not_after;
    populate_dn(&mut ca_params, &config.cert);

    let ca_key = KeyPair::generate()?;
    let ca_cert = ca_params.self_signed(&ca_key)?;

    // Generate end entity
    let mut end_params = CertificateParams::new(vec![config.cert.domain.clone()])?;
    end_params.is_ca = IsCa::NoCa;
    end_params.not_before = now;
    end_params.not_after = not_after;
    populate_dn(&mut end_params, &config.cert);

    let end_key = KeyPair::generate()?;
    let end_cert = end_params.signed_by(&end_key, &ca_cert, &ca_key)?;

    // Write files
    fs::write(config.tls.ca, ca_cert.pem())?;
    fs::write(config.tls.key, end_key.serialize_pem())?;
    fs::write(config.tls.cert, end_cert.pem())?;

    info!("Certificates generated");
    Ok(())
}

fn prompt(name: &str, default: &str) -> Result<String> {
    Ok(Input::new()
        .with_prompt(name)
        .default(default.into())
        .interact_text()?)
}

fn init_config() -> Result<()> {
    let config_path = build_config::path();

    println!("=== General ===");
    let log_level = prompt("Log level", "debug")?;
    let secret = prompt(
        "Secret (session cookie key)",
        &uuid::Uuid::new_v4().to_string(),
    )?;

    println!("\n=== Connection ===");
    let conn_addr = prompt("Client listener address", "0.0.0.0:27853")?;

    println!("\n=== API ===");
    let api_addr = prompt("API address", "0.0.0.0:10801")?;
    let use_https = Confirm::new()
        .with_prompt("Use HTTPS for API?")
        .default(false)
        .interact()?;

    println!("\n=== TLS ===");
    let tls_ca = prompt("CA cert path", "certs/ca.crt")?;
    let tls_key = prompt("TLS key path", "certs/end.key")?;
    let tls_cert = prompt("TLS cert path", "certs/end.crt")?;

    println!("\n=== Certificate Subject ===");
    let country = prompt("Country", "CN")?;
    let state = prompt("State", "ShangHai")?;
    let locality = prompt("Locality", "ShangHai")?;
    let org = prompt("Organization", "m0n1t0r")?;
    let unit = prompt("Unit", ".")?;
    let domain = prompt("Domain", "localhost")?;

    let config = FileConfig {
        general: GeneralConfig { log_level, secret },
        conn: ConnConfig {
            addr: conn_addr.parse().context("invalid conn address")?,
        },
        api: ApiConfig {
            addr: api_addr.parse().context("invalid API address")?,
            use_https,
        },
        tls: TlsConfig {
            key: tls_key.into(),
            cert: tls_cert.into(),
            ca: tls_ca.into(),
        },
        cert: CertConfig {
            country,
            state,
            locality,
            org,
            unit,
            domain,
        },
    };

    let content = toml::to_string_pretty(&config)?;
    fs::write(&config_path, &content)?;
    info!("Config written to {}", config_path.display());

    Ok(())
}

fn main() -> Result<()> {
    Logger::try_with_str("debug")?.start()?;
    let args = Arguments::parse();

    if args.init {
        if build_config::exists() && !args.force {
            warn!("Config already exists. Use --force to overwrite.");
            return Ok(());
        }
        init_config()?;
    }

    if args.cert {
        if build_cert::exists() && !args.force {
            warn!("Certificates already exist. Use --force to overwrite.");
            return Ok(());
        }
        generate_certs()?;
    }

    Ok(())
}
```

Key changes:
- Added `--force` flag to `Arguments`
- Extracted `populate_dn()` helper — eliminates duplicated DN setup for CA and end-entity
- Guard conditions now use `exists()` (renamed from `check()`) with `!args.force` override
- Warning messages tell the user about `--force`

- [ ] **Step 3: Verify it compiles**

```bash
cargo check -p xtask
```

- [ ] **Step 4: Commit**

```bash
git add xtask/Cargo.toml xtask/src/main.rs && git commit -m "refactor: xtask cleanup — remove dead features, add --force, deduplicate cert DN"
```

---

### Task 10: Update documentation

**Files:**
- Modify: `CLAUDE.md`
- Modify: `CODEBASE.md`

- [ ] **Step 1: Update CLAUDE.md**

In `CLAUDE.md`, update the Workspace Structure section. Change:

```markdown
- **m0n1t0r-build** — Build-time utilities (config loading, cert validation, version tracking via vergen, dependency validation)
- **m0n1t0r-macro** — Procedural macros
- **xtask** — Build automation (interactive config generator, cert generation via rcgen)
```

to:

```markdown
- **m0n1t0r-build** — Build toolkit (config loading, cert validation, version tracking via vergen, dependency validation, platform feature validation, Windows resource embedding)
- **xtask** — Build automation (interactive config generator, cert generation via rcgen; use `--force` to overwrite existing files)
```

In the First-Time Setup section, update:

```markdown
### First-Time Setup
```
cargo xtask -i    # interactive config.toml generator
cargo xtask -c    # generate TLS certificates
cargo xtask -i --force  # regenerate config.toml
cargo xtask -c --force  # regenerate TLS certificates
```
```

- [ ] **Step 2: Update CODEBASE.md**

In `CODEBASE.md`, update the workspace layout (lines 7-17). Change:

```
m0n1t0r/
  m0n1t0r-server/    # Server binary (TLS listener + REST/WS API)
  m0n1t0r-client/    # Client agent binary
  m0n1t0r-common/    # Shared RPC traits, types, error definitions
  m0n1t0r-build/     # Build-time config loading, cert validation, version tracking
  m0n1t0r-macro/     # Procedural macros (currently empty)
  m0n1t0r-ui/        # React + TypeScript + Vite + Ant Design dashboard
  xtask/             # Build automation (config generator, cert generator)
  deps/              # Vendored: qqkey (QQ account access), scrap (screen capture + wayland)
```

to:

```
m0n1t0r/
  m0n1t0r-server/    # Server binary (TLS listener + REST/WS API)
  m0n1t0r-client/    # Client agent binary
  m0n1t0r-common/    # Shared RPC traits, types, error definitions
  m0n1t0r-build/     # Build toolkit (config, cert, version, dep, platform, winres)
  m0n1t0r-ui/        # React + TypeScript + Vite + Ant Design dashboard
  xtask/             # Build automation (config generator, cert generator; --force to overwrite)
  deps/              # Vendored: qqkey (QQ account access), scrap (screen capture + wayland)
```

Update the m0n1t0r-build section (lines 487-493). Change:

```markdown
- **`config.rs`** — `FileConfig` struct matching `config.toml`. Provides `path()`, `ensure()` (panics if missing), `check()`, `read()`. Config is read at compile time to bake values into binaries.
- **`cert.rs`** — `path()` returns `[ca, cert, key]` paths. `ensure()` panics if certs missing. `check()` returns whether any are missing.
- **`version.rs`** — Calls `vergen` to emit build metadata env vars.
- **`dep.rs`** — `check_xmake()`, `check_xrepo()`, `xrepo_fetch(dep)` — validates and fetches native dependencies.
```

to:

```markdown
- **`config.rs`** — `FileConfig` struct matching `config.toml`. Provides `path()`, `ensure()` (panics if missing), `exists()`, `read()`.
- **`cert.rs`** — `paths()` returns `[ca, cert, key]` paths. `ensure()` panics if certs missing. `exists()` returns true when all certs are present.
- **`version.rs`** — Calls `vergen` to emit build metadata env vars.
- **`dep.rs`** — `check_xmake()`, `check_xrepo()`, `xrepo_fetch(dep)` — validates and fetches native dependencies.
- **`platform.rs`** — `ensure()` validates that a platform feature flag is set at build time.
- **`winres.rs`** — `embed(icon, manifest)` embeds Windows resource manifest with icon (behind `winres` feature).
```

Update the xtask section (lines 498-502). Change:

```markdown
- **`-i` / `--init`** — Interactive `config.toml` generator. Prompts for all config fields with sensible defaults. Won't overwrite existing valid config.
- **`-c` / `--cert`** — TLS certificate generator using `rcgen`. Creates a self-signed CA + end-entity cert pair (10-year validity). Writes to paths specified in config.
```

to:

```markdown
- **`-i` / `--init`** — Interactive `config.toml` generator. Prompts for all config fields with sensible defaults. Warns and exits if config exists (use `--force` to overwrite).
- **`-c` / `--cert`** — TLS certificate generator using `rcgen`. Creates a self-signed CA + end-entity cert pair (10-year validity). Warns and exits if certs exist (use `--force` to overwrite).
- **`-f` / `--force`** — Force overwrite existing config/certs.
```

Remove the m0n1t0r-macro reference from CODEBASE.md line 13 (`m0n1t0r-macro/     # Procedural macros (currently empty)`).

- [ ] **Step 3: Commit**

```bash
git add CLAUDE.md CODEBASE.md && git commit -m "docs: update CLAUDE.md and CODEBASE.md for build system refactor"
```

---

### Task 11: Final verification

- [ ] **Step 1: Run cargo check on the whole workspace**

```bash
cargo check --workspace --features winnt
```

Expected: Clean compilation with no warnings related to build scripts.

- [ ] **Step 2: Verify xtask --help shows --force flag**

```bash
cargo xtask --help
```

Expected output includes `-f, --force  Force overwrite existing files`.

- [ ] **Step 3: Verify m0n1t0r-macro is fully gone**

```bash
ls m0n1t0r-macro/ 2>&1
```

Expected: "No such file or directory" or similar error.
