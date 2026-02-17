ARG FLUTTER_VERSION=3.27.4
FROM instrumentisto/flutter:${FLUTTER_VERSION}

# Configurable version parameters
ARG RUST_VERSION=1.88.0

# Environment variables
ENV DEBIAN_FRONTEND=noninteractive
ENV CARGO_TERM_COLOR=always

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    ninja-build \
    pkg-config \
    libclang-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
SHELL ["/bin/bash", "-c"]
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain ${RUST_VERSION} && \
    source ~/.cargo/env && \
    rustup toolchain install nightly && \
    rustup component add rustfmt clippy && \
    rustup component add rustfmt --toolchain nightly && \
    rustup target add wasm32-unknown-unknown --toolchain nightly
ENV PATH="/root/.cargo/bin:${PATH}"

# Working directory
WORKDIR /workspace

# Default command
CMD ["bash"]
