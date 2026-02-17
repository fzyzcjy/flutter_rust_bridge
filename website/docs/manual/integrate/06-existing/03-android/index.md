import V1Notice from '@site/docs/snippets/_v1-notice.mdx';

# Integrating with Android

<V1Notice />

The setup process is identical to [Android setup](../template/setup/android),
so go ahead and follow the steps described there. Once you're done, we will discuss
how to modify the existing toolchain to accommodate Rust.

There is more than one way to set up Cargo to run alongside Gradle, so this guide
will cover the two main ones: hooking onto tasks, and integrating with CMake.
