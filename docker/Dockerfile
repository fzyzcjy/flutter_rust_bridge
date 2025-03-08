FROM rust:1.84.0-bookworm

# https://github.com/instrumentisto/flutter-docker-image/blob/main/Dockerfile
COPY --from=instrumentisto/flutter:3.29.1 /usr/local/flutter /usr/local/flutter
ENV PATH=$PATH:/usr/local/flutter/bin:/usr/local/flutter/bin/cache/dart-sdk/bin

SHELL ["/bin/bash", "-c"]

RUN \
    set -euxo pipefail ; \
    mkdir /workspace ;
#    curl -fsSL https://fvm.app/install.sh | bash ; \
#    fvm global --force 3.29.1 ; \
#    flutter --version ; fvm flutter --version ; cargo --version ; rustc --version

WORKDIR "/workspace"
