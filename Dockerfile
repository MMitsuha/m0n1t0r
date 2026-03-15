# =============================================================================
# Stage 1: Build the server binary
# =============================================================================
FROM ghcr.io/rust-lang/rust:nightly-trixie-slim AS server-builder

WORKDIR /app

# Install system dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    g++ gcc git curl wget nasm yasm clang cmake make ninja-build pkg-config \
    libclang-dev libssl-dev ca-certificates \
    libgtk-3-dev libxcb-randr0-dev libxdo-dev libxfixes-dev \
    libxcb-shape0-dev libxcb-xfixes0-dev libasound2-dev libpulse-dev \
    libgstreamer1.0-dev libgstreamer-plugins-base1.0-dev libpam0g-dev \
    libavutil-dev libavcodec-dev libavdevice-dev libavfilter-dev \
    libavformat-dev libswresample-dev libswscale-dev \
    autoconf automake libtool build-essential curl zip unzip tar \
    && rm -rf /var/lib/apt/lists/*

# Install cxxbridge-cmd
RUN cargo install cxxbridge-cmd

# Set up vcpkg and install native dependencies
RUN git clone --depth 1 https://github.com/microsoft/vcpkg /opt/vcpkg \
    && /opt/vcpkg/bootstrap-vcpkg.sh
ENV VCPKG_ROOT=/opt/vcpkg
RUN /opt/vcpkg/vcpkg install libvpx libyuv opus aom ffmpeg

# Copy source tree
COPY . .

# Use example config for build (config.toml is gitignored)
RUN cp config.example.toml config.toml

# Generate TLS certificates and build server
RUN cargo xtask -c
RUN cargo build --release --features linux,rd --bin m0n1t0r-server

# =============================================================================
# Stage 2: Build the UI static assets
# =============================================================================
FROM oven/bun:latest AS ui-builder

WORKDIR /app

COPY m0n1t0r-ui/package.json m0n1t0r-ui/bun.lock ./
RUN bun install --frozen-lockfile

COPY m0n1t0r-ui/ .
RUN bun run build

# =============================================================================
# Stage 3: Final runtime image
# =============================================================================
FROM debian:trixie-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates nginx \
    libgtk-3-0 libxcb-randr0 libxdo3 libxfixes3 \
    libxcb-shape0 libxcb-xfixes0 libasound2t64 libpulse0 \
    libgstreamer1.0-0 libgstreamer-plugins-base1.0-0 libpam0g \
    ffmpeg \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy server binary and config
COPY --from=server-builder /app/target/release/m0n1t0r-server /app/m0n1t0r-server
COPY --from=server-builder /app/config.toml /app/config.toml

# Copy generated certificates
COPY --from=server-builder /app/certs /app/certs

# Copy UI static assets
COPY --from=ui-builder /app/dist /app/ui

# Copy nginx config and entrypoint
COPY nginx.conf /etc/nginx/nginx.conf
COPY entrypoint.sh /app/entrypoint.sh
RUN chmod +x /app/entrypoint.sh

# Expose ports:
#   80    - UI (nginx reverse proxy)
#   27853 - Client TLS connections
EXPOSE 80 27853

ENTRYPOINT ["/app/entrypoint.sh"]
