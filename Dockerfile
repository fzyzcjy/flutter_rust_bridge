FROM rust:1.70.0 as builder
WORKDIR /usr/src/flutter_rust_bridge_codegen
COPY . .
RUN cargo install --path frb_codegen &&  \
    ls -al /usr/local/cargo/bin/

FROM debian:bullseye-slim
# TODO
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/flutter_rust_bridge_codegen /usr/local/bin/flutter_rust_bridge_codegen
CMD ["flutter_rust_bridge_codegen"]
