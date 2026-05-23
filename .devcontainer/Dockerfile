ARG FLUTTER_VERSION=3.44.0
FROM ghcr.io/cirruslabs/flutter:${FLUTTER_VERSION}

# Configurable version parameters
ARG RUST_VERSION=1.93.1
ARG RUST_NIGHTLY_VERSION=2025-02-01
ARG NODE_VERSION=22.11.0

# Environment variables
ENV DEBIAN_FRONTEND=noninteractive
ENV CARGO_TERM_COLOR=always
ENV PIP_BREAK_SYSTEM_PACKAGES=1

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    clang \
    cmake \
    ninja-build \
    pkg-config \
    libclang-dev \
    libgtk-3-dev \
    liblzma-dev \
    libstdc++-12-dev \
    mesa-utils \
    python3-pip \
    xz-utils \
    && rm -rf /var/lib/apt/lists/*

# Configure pip to allow system package installation (PEP 668)
RUN mkdir -p /etc/pip && \
    echo "[global]\nbreak-system-packages = true" > /etc/pip/pip.conf

# Install Rust
SHELL ["/bin/bash", "-c"]
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain ${RUST_VERSION} && \
    export PATH="$HOME/.cargo/bin:$PATH" && \
    rustup component add rustfmt clippy llvm-tools-preview && \
    rustup target add wasm32-unknown-unknown && \
    rustup toolchain install "nightly-${RUST_NIGHTLY_VERSION}" && \
    rustup component add rust-src llvm-tools-preview --toolchain "nightly-${RUST_NIGHTLY_VERSION}" && \
    rustup target add wasm32-unknown-unknown --toolchain "nightly-${RUST_NIGHTLY_VERSION}" && \
    rustup toolchain install nightly && \
    rustup component add rust-src --toolchain nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly
ENV PATH="/root/.cargo/bin:${PATH}"

# Install Node.js and npm from official multi-arch binaries.
RUN ARCH="$(dpkg --print-architecture)" && \
    case "$ARCH" in \
        amd64) NODE_ARCH="x64" ;; \
        arm64) NODE_ARCH="arm64" ;; \
        *) echo "Unsupported architecture: $ARCH" >&2; exit 1 ;; \
    esac && \
    curl -fsSL "https://nodejs.org/dist/v${NODE_VERSION}/node-v${NODE_VERSION}-linux-${NODE_ARCH}.tar.xz" \
        | tar -xJ -C /usr/local --strip-components=1

# Install wasm-pack for web tests (using cargo for better reliability)
RUN cargo install wasm-pack

# Install extra CLI tools used by internal generation and coverage workflows
RUN cargo install cargo-expand && \
    cargo install cargo-llvm-cov && \
    npm install -g all-contributors-cli

# Install Chromium for web tests (headless testing). Google Chrome does not
# provide Linux arm64 builds yet, and Ubuntu's chromium-browser package is snap.
ARG PLAYWRIGHT_VERSION=1.59.1
ENV PLAYWRIGHT_BROWSERS_PATH=/opt/ms-playwright
RUN npm install -g "playwright@${PLAYWRIGHT_VERSION}" && \
    playwright install --with-deps chromium && \
    ln -s "$(find "${PLAYWRIGHT_BROWSERS_PATH}" -path "*/chrome-linux*/chrome" -type f | head -n 1)" /usr/bin/google-chrome
ENV CHROME_BIN=/usr/bin/google-chrome

# Working directory
WORKDIR /workspace

RUN flutter --version && \
    dart --version && \
    node --version && \
    npm --version && \
    cargo --version && \
    rustup toolchain list | grep "nightly-${RUST_NIGHTLY_VERSION}" && \
    rustup target list --installed | grep wasm32-unknown-unknown && \
    rustup target list --toolchain "nightly-${RUST_NIGHTLY_VERSION}" --installed | grep wasm32-unknown-unknown && \
    rustup component list --installed | grep llvm-tools && \
    rustup component list --toolchain nightly --installed | grep rust-src && \
    cargo expand --version && \
    cargo llvm-cov --version && \
    wasm-pack --version && \
    all-contributors --version && \
    "${CHROME_BIN}" --version

# Default command
CMD ["bash"]
