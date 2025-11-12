# m0n1t0r RAT

[![MacOS Build](https://github.com/MMitsuha/m0n1t0r/actions/workflows/macos.yml/badge.svg)](https://github.com/MMitsuha/m0n1t0r/actions/workflows/macos.yml)
[![Ubuntu Build](https://github.com/MMitsuha/m0n1t0r/actions/workflows/ubuntu.yml/badge.svg)](https://github.com/MMitsuha/m0n1t0r/actions/workflows/ubuntu.yml)
[![Windows Build](https://github.com/MMitsuha/m0n1t0r/actions/workflows/windows.yml/badge.svg)](https://github.com/MMitsuha/m0n1t0r/actions/workflows/windows.yml)

*`m0n1t0r` is a high performance and high reliability command and control tool set for red teams*

## Features

1. File operations
2. Socks5 proxy
3. Command execution
4. Restful web api control
5. Remote screen monitor
6. Shellcode execution

## Build

- Ubuntu

```bash
sudo apt install -y zip g++ gcc git curl wget nasm yasm libgtk-3-dev clang libxcb-randr0-dev libxdo-dev libxfixes-dev libxcb-shape0-dev libxcb-xfixes0-dev libasound2-dev libpulse-dev cmake make libclang-dev ninja-build libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev libpam0g-dev xmake pkg-config libavutil-dev libavcodec-dev libavdevice-dev libavfilter-dev libavformat-dev libswresample-dev libswscale-dev
git clone https://github.com/microsoft/vcpkg
./vcpkg/bootstrap-vcpkg.sh
export VCPKG_ROOT=$PWD/vcpkg
./vcpkg/vcpkg install libvpx libyuv opus aom mfx-dispatch ffmpeg
cargo install cxxbridge-cmd
cargo xtask -c
cargo build --bin m0n1t0r-server -r && cargo build --bin m0n1t0r-client --features linux -r
```

- Windows

```powershell
git clone https://github.com/microsoft/vcpkg
./vcpkg/bootstrap-vcpkg.bat
export VCPKG_ROOT=$PWD/vcpkg
./vcpkg/vcpkg install libvpx:x64-windows-static libyuv:x64-windows-static opus:x64-windows-static aom:x64-windows-static mfx-dispatch:x64-windows-static ffmpeg:x64-windows-static
scoop install main/xmake
cargo install cxxbridge-cmd
cargo xtask -c
cargo build --bin m0n1t0r-server -r && cargo build --bin m0n1t0r-client --features winnt -r
```

- MacOS

```zsh
brew install binutils meson nasm ninja autoconf automake autoconf-archive python3 libtool cmake gcc wget pkg-config libresample ffmpeg
git clone https://github.com/microsoft/vcpkg
./vcpkg/bootstrap-vcpkg.sh
export VCPKG_ROOT=$PWD/vcpkg
./vcpkg/vcpkg install libvpx libyuv opus aom ffmpeg
```

## Roadmap

1. Add remote desktop
2. etc...

## License

`m0n1t0r` is distribute in GPLv3 license. The software comes absolutely no warranty.
