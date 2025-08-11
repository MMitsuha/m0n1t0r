# m0n1t0r RAT

*`m0n1t0r` is a high performance and high reliability command and control tool set for red teams*

## Features

1. File operations
2. Socks5 proxy
3. Command execution
4. Restful web api control
5. ~~Screen monitor~~ (removed due to unstable performance on different platforms)
6. Shellcode execution

## Build

- Unix

1. Install `xmake`, `libpipewire-0.3-dev`, `clang`, `libdbus-glib-1-dev`, `libclang-dev`, `libspa-0.2-dev`, `libboost-all-dev`, etc. (see more in `.github/workflows/ubuntu.yml`) using your package management tool (such as `apt` or `pacman`)
2. Install `cxxbridge` using `cargo install cxxbridge-cmd`
3. Install `vcpkg`
4. Install `Qt` using `aqt`
5. Run `cargo xtask -c` to generate certs if needed
6. Run `cargo build` to build a general client and server or use `cargo build --features linux` to build a Linux specific client and server

- Windows

1. Install build toolchain such as `msvc` or `gcc` (usually automatically done by `rustup`)
2. Install `cxxbridge` using `cargo install cxxbridge-cmd`
3. Install `xmake` and `msys2`
4. Run `cargo xtask -c` in `msys2` to generate certs if needed
5. Run `cargo build` to build client and server or use `cargo build --features windows` to build a Windows specific client and server

## Roadmap

1. Add remote desktop
2. etc...

## License

`m0n1t0r` is distribute in GPLv3 license. The software comes absolutely no warranty.
