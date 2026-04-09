# Build Scripts Refactor Design

## Goal

Restructure the build system to fix bugs, remove dead code, deduplicate shared logic, and reorganize responsibilities so m0n1t0r-build is the central build toolkit and build.rs files are thin orchestrators.

Breaking changes are allowed. Docs will be updated.

## Changes

### 1. m0n1t0r-build: Expand into build toolkit

**New module layout:**

| Module | Responsibility |
|--------|---------------|
| `config` | Config TOML parsing, validation, path resolution |
| `cert` | Cert path resolution, existence check, rerun-if-changed |
| `version` | vergen build metadata emission |
| `dep` | xmake/xrepo availability checks, xrepo fetch parsing |
| `platform` | Platform feature compile_error (shared across crates) |
| `winres` | Windows manifest/icon embedding (shared across crates) |

**Naming convention fix (cert + config):**
- `exists() -> bool` — true if the resource exists and is valid
- `ensure()` — panics with a helpful message if `!exists()`
- `read() -> T` — parse and return the resource

Currently `cert::check()` returns true when certs are MISSING, while `config::check()` returns true when config EXISTS. Both will be renamed to `exists()` with consistent "true = present and valid" semantics.

**cert::ensure() bug fix:**
Current code calls `check()` (true = missing), enters the panic branch, but then falls through to emit `rerun-if-changed` for missing files. Fix: gate rerun-if-changed behind `exists()` being true.

**Dependency cleanup:**
- Remove `nom` (unused)
- Replace `execute` crate with `std::process::Command` in `dep.rs`

**New module: `platform`**
Provides a macro or compile_error block that crates can invoke to validate platform features. Eliminates the copy-pasted `#[cfg(not(any(...)))] compile_error!()` in client and server build.rs.

**New module: `winres`**
Provides a function like `embed(icon_path, manifest: Option<&str>)` that handles:
- Creating `winres::WindowsResource`
- Setting icon and language
- Optionally setting a manifest (client uses UAC manifest, server does not)
- Calling `res.compile()`

This replaces the duplicated `add_manifest_windows()` functions in client and server build.rs.

### 2. Thin build.rs files

**m0n1t0r-client/build.rs** (from 92 lines to ~20):
```rust
m0n1t0r_build::platform::ensure(); // compile_error if no platform feature
config::ensure();
cert::ensure();
// emit env vars from config
// cxx_build::bridge() calls
// xmake_build() calls (kept inline, client-specific)
m0n1t0r_build::winres::embed(icon, Some(UAC_MANIFEST)); // Windows only
```

**m0n1t0r-server/build.rs** (from 31 lines to ~5):
```rust
m0n1t0r_build::platform::ensure();
m0n1t0r_build::winres::embed(icon, None); // Windows only
```

**m0n1t0r-common/build.rs** (unchanged):
```rust
version::generate();
cert::ensure();
```

### 3. xtask cleanup

**Remove unused features:** `general`, `winnt`, `linux`, `macos` in xtask/Cargo.toml — they don't gate anything.

**Add `--force` flag:**
- `cargo xtask -i` — if config.toml exists and `--force` not set, print warning and exit
- `cargo xtask -c` — if certs exist and `--force` not set, print warning and exit
- With `--force`, overwrite without prompting

**Deduplicate cert DN setup:**
Extract a helper function that populates DN fields on a `CertificateParams` from `CertConfig`. Called for both CA and end-entity cert generation.

### 4. Remove m0n1t0r-macro

- Delete `m0n1t0r-macro/` directory
- Remove from workspace members in root `Cargo.toml`
- The crate is empty (lib.rs has no macros) and adds build overhead

### 5. Doc updates

**CLAUDE.md:**
- Remove m0n1t0r-macro from workspace structure
- Add `platform` and `winres` modules to m0n1t0r-build description
- Document `--force` flag for xtask
- Update build.rs descriptions

**CODEBASE.md:**
- Same updates as CLAUDE.md where applicable

## Out of scope

- deps/scrap/build.rs — vendored dependency, not touching
- xmake lua files — working fine, no changes needed
- .cargo/config.toml — platform linking flags are correct
- m0n1t0r-ui build — not Rust build system
