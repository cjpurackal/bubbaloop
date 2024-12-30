#!/bin/bash

set -eu

sudo apt-get update && sudo apt-get install -y \
    build-essential \
    cmake \
    curl \
    libgstreamer1.0-dev \
    libgstreamer-plugins-base1.0-dev \
    nasm
