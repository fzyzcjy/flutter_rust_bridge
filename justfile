default:
    @echo 'Please use ./frb_internal (or ./frb_internal.bat) instead.'

# Please put most scripts in `frb_internal`.
# here are only some ad-hoc scripts, seldomly executed ones,
# or lightweight scripts that is much easier to implement with justfile

# Execute the in-tree version of `flutter_rust_bridge_codegen`
flutter_rust_bridge_codegen *args:
    cd {{invocation_directory()}} && \
        cargo run \
        --manifest-path {{justfile_directory()}}/frb_codegen/Cargo.toml \
        -- {{args}}

# rsync code from my host to VM
_rsync_ubuntu:
    #!/usr/bin/env bash
    set -euxo pipefail
    function run_rsync() {
        rsync -rtDvPL --filter=':- .gitignore' --exclude='.git' \
            --no-p --no-g --chmod=ugo=rwX \
            ./ ubuntu:~/flutter_rust_bridge
    }
    run_rsync; fswatch -o ./ | while read f; do run_rsync; done
