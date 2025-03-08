FROM rust:1.84.0-bookworm

ENTRYPOINT "/bin/bash"

RUN \
    set -euxo pipefail ; \
    curl -fsSL https://fvm.app/install.sh | bash ; \
    fvm global --force 3.29.1 ; \
    flutter --version ; fvm flutter --version ; cargo --version ; rustc --version
