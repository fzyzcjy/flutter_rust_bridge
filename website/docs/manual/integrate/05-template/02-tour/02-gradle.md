import V1Notice from '@site/docs/snippets/_v1-notice.mdx';

# `android/app/build.gradle`

<V1Notice />

This file is part of the default Flutter build process for Android apps.
The template injects additional hooks to run [`cargo-ndk`](https://lib.rs/crates/cargo-ndk)
upon invoking `flutter run`. This method is explained more in detail in
[Hooking onto tasks](../../existing/android/tasks).
