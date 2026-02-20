ARG FLUTTER_VERSION=3.27.4
FROM instrumentisto/flutter:${FLUTTER_VERSION}

# Configurable version parameters
ARG RUST_VERSION=1.88.0

# Environment variables
ENV DEBIAN_FRONTEND=noninteractive
ENV CARGO_TERM_COLOR=always
ENV PIP_BREAK_SYSTEM_PACKAGES=1

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    ninja-build \
    pkg-config \
    libclang-dev \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

# Configure pip to allow system package installation (PEP 668)
RUN mkdir -p /etc/pip && \
    echo "[global]\nbreak-system-packages = true" > /etc/pip/pip.conf

# Install Rust
SHELL ["/bin/bash", "-c"]
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain ${RUST_VERSION} && \
    export PATH="$HOME/.cargo/bin:$PATH" && \
    rustup component add rustfmt clippy && \
    rustup toolchain install nightly-2025-02-01 && \
    rustup component add rust-src --toolchain nightly-2025-02-01 && \
    rustup target add wasm32-unknown-unknown --toolchain nightly-2025-02-01
ENV PATH="/root/.cargo/bin:${PATH}"

# Install wasm-pack for web tests
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install Chrome for web tests (headless testing)
RUN apt-get update && apt-get install -y \
    chromium \
    chromium-driver \
    && rm -rf /var/lib/apt/lists/*
ENV CHROME_BIN=/usr/bin/chromium

# Working directory
WORKDIR /workspace

# Default command
CMD ["bash"]
