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
5. Run `./certs/generate.sh` if needed
6. Run `cargo build` to build a general client and server or use `cargo build --features windows` to build a Windows specific client and server
7. Build sdk using `cd ./m0n1t0r-sdk && xmake f -m release -y -v && xmake build -y -v && xmake package && cd ..`
8. Build UI using `cd ./m0n1t0r-ui && xmake f -m release --qt=<PATH_TO_QT>/qt/ -v -y && xmake build -y -v && cd ..`

- Windows

1. Install build toolchain such as `msvc` or `gcc` (usually automatically done by `rustup`)
2. Install `cxxbridge` using `cargo install cxxbridge-cmd`
3. Install `xmake` and `msys2`
4. Run `./certs/generate.sh` in `msys2` if needed
5. Run `cargo build` to build client and server

*WARNING: Due to `boost` build failure in Windows, you have to figure out a way to install `boost` manually and then continue to build UI*

## Roadmap

1. Add remote desktop
2. etc...

## License

`m0n1t0r` is distribute in GPLv3 license. The software comes absolutely no warranty.
