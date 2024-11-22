# m0n1t0r RAT

*`m0n1t0r` is a high performance and high reliability command and control tool set for red teams*

## Features

1. File operations
2. Socks5 proxy
3. Command execution
4. Restful web api control
5. Screen monitor

## Build

- Unix

1. Install `xmake`, `libpipewire-0.3-dev`, `clang`, `libdbus-glib-1-dev`, `libclang-dev`, `libspa-0.2-dev` using your package management tool (such as `apt` or `pacman`)
2. Run `./certs/generate.sh` if needed
3. Run `cargo build`

- Windows

1. Install build toolchain such as `msvc` or `gcc` (usually automatically done by `rustup`)
2. Install `xmake` and `msys2`
3. Run `./certs/generate.sh` in `msys2` if needed
4. Run `cargo build`

## Roadmap

1. Add shellcode execution
2. Add remote desktop
3. etc...

## License

`m0n1t0r` is distribute in GPLv3 license. The software comes absolutely no warranty.
