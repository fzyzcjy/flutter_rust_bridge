FROM rust:1.84.0-bookworm

# https://github.com/instrumentisto/flutter-docker-image/blob/main/Dockerfile
COPY --from=instrumentisto/flutter:3.29.1 /usr/local/flutter /usr/local/flutter
ENV PATH=$PATH:/usr/local/flutter/bin:/usr/local/flutter/bin/cache/dart-sdk/bin

SHELL ["/bin/bash", "-c"]

RUN \
    set -euxo pipefail ; \
    mkdir /workspace ; \
    git config --global --add safe.directory /usr/local/flutter ; \
    dart --disable-analytics ; \
    flutter --version ; cargo --version ; rustc --version

WORKDIR "/workspace"
