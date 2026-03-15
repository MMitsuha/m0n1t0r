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

### Generate TLS Certificates (required before first build)
```
cargo xtask -c
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
Default ports: `0.0.0.0:27853` (client TLS connections), `0.0.0.0:10801` (REST/WebSocket API).

### Client
In debug mode, connects to `127.0.0.1:27853`. In release mode, server address is baked in via `M0N1T0R_DOMAIN` env var at compile time.

## Architecture

### Workspace Structure
- **m0n1t0r-server** — Actix-web REST/WebSocket API + TLS listener for client connections
- **m0n1t0r-client** — Agent binary that connects back to the server over TLS
- **m0n1t0r-common** — Shared types, RPC trait definitions, error types (the contract between server and client)
- **m0n1t0r-ui** — React + TypeScript + Vite + Ant Design web dashboard
- **m0n1t0r-build** — Build-time utilities (cert generation, version tracking via vergen, dependency validation)
- **m0n1t0r-macro** — Procedural macros
- **xtask** — Build automation (cert generation)
- **deps/** — Vendored dependencies (qqkey, scrap with wayland support)

### Communication Model
1. **Client → Server**: TLS connection on port 27853, bidirectional async RPC via `remoc` crate with MessagePack serialization
2. **UI → Server**: HTTP REST API + WebSocket on port 10801 at `/api/v1/`
3. Server maintains a `ServerMap` (slotmap-based) tracking connected clients as `ServerObj` instances
4. API handlers look up clients in the map and invoke RPC methods through remoc channels

### Server API Layout (`m0n1t0r-server/src/web/api/`)
- `client/` — Per-client endpoints: fs, process, proxy, rd (remote desktop), qq, update, autorun
- `server/` — Server-wide: notifications, proxy list
- `session/` — Authentication (TODO)
- `global/` — Server info, version
- Response envelope: `{ code: int, body: T }`

### Client Platform Code (`m0n1t0r-client/src/client/`)
- `general/` — Cross-platform handlers
- `windows/` — Win32-specific (blind/ETW patching, autorun, filesystem charset)
- `unix/` — Linux/macOS-specific
- Platform dispatch uses Cargo features and `cfg_block`

### Key Build-Time Environment Variables
- `M0N1T0R_DOMAIN` — Server address baked into release client binary
- `M0N1T0R_COUNTRY`, `M0N1T0R_STATE`, `M0N1T0R_LOCALITY`, `M0N1T0R_ORG`, `M0N1T0R_UNIT` — TLS cert subject fields

### Server Runtime Configuration (`config.toml`)
- `conn_addr` — Client TLS listener address (default: `0.0.0.0:27853`)
- `api_addr` — REST/WebSocket API address (default: `0.0.0.0:10801`)
- `key` — TLS private key path (required)
- `cert` — TLS certificate path (required)
- `use_https` — Use TLS for API server (default: `false`)
- `log_level` — Logging verbosity (default: `debug`)
- `secret` — Session cookie signing key (required)

### Key Dependencies
- **tokio** — Async runtime
- **actix-web** — HTTP server (with rustls, secure cookies, WebSocket via actix-ws)
- **remoc** — Async RPC framework (MessagePack codec over TLS)
- **rustls** — TLS implementation (both server and client)
- **cxx** — C++ FFI for platform-specific native code
- **ffmpeg-next** — Video encoding for remote desktop
- **scrap** — Screen capture (vendored, with wayland support)

### Release Profile
Binaries are optimized for size: `opt-level = "z"`, LTO enabled, single codegen unit, symbols stripped, panic=abort.
