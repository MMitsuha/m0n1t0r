# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

m0n1t0r is a cross-platform C2 (command and control) framework written in Rust. It consists of a server, client agent, shared library, and a React web dashboard.

## Build Commands

### Prerequisites
- Rust 1.85+ (edition 2024)
- xmake (C/C++ build system)
- cxxbridge-cmd: `cargo install cxxbridge-cmd`
- vcpkg with packages: `libvpx libyuv opus aom ffmpeg`
- System: `binutils meson nasm ninja autoconf automake cmake pkg-config ffmpeg` (via brew on macOS)

### First-Time Setup
```
cargo xtask -i    # interactive config.toml generator
cargo xtask -c    # generate TLS certificates
```

### Build Rust Binaries
Platform feature flags are **required** and mutually exclusive: `macos`, `linux`, `winnt`, `winnt-uac`
Optional feature: `rd` (remote desktop — enables ffmpeg, scrap, hbb_common dependencies)
```
cargo build --bin m0n1t0r-server --features macos,rd -r
cargo build --bin m0n1t0r-client --features macos,rd -r
```

### Build UI
```
cd m0n1t0r-ui && bun install && bun run build
```

### Lint UI
```
cd m0n1t0r-ui && bun run lint
```

### Run Server
```
./target/release/m0n1t0r-server [config.toml]
```
Configuration is read from a TOML file (defaults to `config.toml` in the working directory).

### Client
In debug mode, connects to `127.0.0.1`. In release mode, server address and port are baked in from `config.toml` `[cert].domain` and `[conn].addr` at compile time.

## Architecture

### Workspace Structure
- **m0n1t0r-server** — Actix-web REST/WebSocket API + TLS listener for client connections
- **m0n1t0r-client** — Agent binary that connects back to the server over TLS
- **m0n1t0r-common** — Shared types, RPC trait definitions, error types (the contract between server and client)
- **m0n1t0r-ui** — React + TypeScript + Vite + Ant Design web dashboard
- **m0n1t0r-build** — Build-time utilities (config loading, cert validation, version tracking via vergen, dependency validation)
- **m0n1t0r-macro** — Procedural macros
- **xtask** — Build automation (interactive config generator, cert generation via rcgen)
- **deps/** — Vendored dependencies (qqkey, scrap with wayland support)

### Communication Model
1. **Client → Server**: TLS connection (port configurable via `[conn].addr`), bidirectional async RPC via `remoc` crate with MessagePack serialization
2. **UI → Server**: HTTP REST API + WebSocket (port configurable via `[api].addr`) at `/api/v1/`
3. Server maintains a `ServerMap` tracking connected clients as `ServerObj` instances
4. API handlers look up clients in the map and invoke RPC methods through remoc channels

### Server API Layout (`m0n1t0r-server/src/web/api/`)
- `client/` — Per-client endpoints: fs, process, proxy, rd (remote desktop, requires `rd` feature), qq, update, autorun
- `server/` — Server-wide: notifications, proxy list
- `session/` — Authentication (TODO)
- `global/` — Server info, version
- Response envelope: `{ code: int, body: T }`

### Client Platform Code (`m0n1t0r-client/src/client/`)
- `general/` — Cross-platform handlers
- `windows/` — Win32-specific (blind/ETW patching, autorun, filesystem charset)
- `unix/` — Linux/macOS-specific
- Platform dispatch uses Cargo features and `cfg_block`

### Configuration (`config.toml`)
Generated interactively via `cargo xtask -i`. See `config.example.toml` for reference.
- **`[general]`** — `log_level` (default: `debug`), `secret` (session cookie signing key)
- **`[conn]`** — `addr` (default: `0.0.0.0:27853`, client TLS listener)
- **`[api]`** — `addr` (default: `0.0.0.0:10801`, REST/WebSocket API), `use_https` (default: `false`)
- **`[tls]`** — `key`, `cert` (PEM file paths for TLS)
- **`[cert]`** — `country`, `state`, `locality`, `org`, `unit`, `domain` (used by `cargo xtask -c` to generate TLS certs; `domain` is also baked into client binary)

### Key Dependencies
- **tokio** — Async runtime
- **actix-web** — HTTP server (with rustls, secure cookies, WebSocket via actix-ws)
- **remoc** — Async RPC framework (MessagePack codec over TLS)
- **rustls** — TLS implementation (both server and client)
- **rcgen** — Pure Rust TLS certificate generation (xtask)
- **cxx** — C++ FFI for platform-specific native code
- **ffmpeg-next** — Video encoding for remote desktop (optional, behind `rd` feature)
- **scrap** — Screen capture (vendored, with wayland support, optional behind `rd` feature)

### Release Profile
Binaries are optimized for size: `opt-level = "z"`, LTO enabled, single codegen unit, symbols stripped, panic=abort.
