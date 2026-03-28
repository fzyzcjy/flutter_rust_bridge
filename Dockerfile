# tentative experiment. create an issue if you want flutter_rust_bridge_codegen bundled in docker!

FROM rust:1.70.0 as builder
WORKDIR /usr/src/flutter_rust_bridge_codegen
COPY . .
RUN cargo install --path frb_codegen &&  \
    ls -al /usr/local/cargo/bin/

FROM dart:3.1.5

# libclang-dev is required by https://pub.dev/packages/ffigen
RUN dart pub global activate ffigen --version 8.0.0 && \
    apt-get update && \
    apt-get install -y ca-certificates gnupg libclang-dev wget && \
    wget -qO- https://dl.google.com/linux/linux_signing_key.pub | \
      gpg --dearmor -o /usr/share/keyrings/google-chrome.gpg && \
    echo "deb [arch=amd64 signed-by=/usr/share/keyrings/google-chrome.gpg] https://dl.google.com/linux/chrome/deb/ stable main" > \
      /etc/apt/sources.list.d/google-chrome.list && \
    apt-get update && \
    apt-get install -y google-chrome-stable && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/flutter_rust_bridge_codegen /usr/local/bin/flutter_rust_bridge_codegen
CMD ["flutter_rust_bridge_codegen"]
