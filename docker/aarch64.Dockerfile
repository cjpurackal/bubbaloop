FROM ghcr.io/cross-rs/aarch64-unknown-linux-gnu:edge

RUN dpkg --add-architecture arm64

RUN apt-get update && apt-get install --assume-yes \
    cmake \
    nasm \
    libgstreamer1.0-dev:arm64 \
    libgstreamer-plugins-base1.0-dev:arm64 \
    libssl-dev:arm64 \
    libglib2.0-dev:arm64 \
    && \
    apt-get clean
