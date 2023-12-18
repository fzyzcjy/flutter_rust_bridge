# Android setup

Before trying this, ensure you can run the example project.

## Rust targets

If you have not already done so, cross-compiling to Android requires some additional
targets which can easily be added:

```shell
rustup target add \
    aarch64-linux-android \
    armv7-linux-androideabi \
    x86_64-linux-android \
    i686-linux-android
```