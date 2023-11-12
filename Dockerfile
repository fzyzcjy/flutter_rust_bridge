FROM rust:1.70.0 as builder
WORKDIR /usr/src/flutter_rust_bridge_codegen
COPY . .
RUN cargo install --path frb_codegen

FROM debian:bullseye-slim
RUN apt-get update
# TODO
# && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/flutter_rust_bridge_codegen /usr/local/bin/flutter_rust_bridge_codegen
CMD ["flutter_rust_bridge_codegen"]
