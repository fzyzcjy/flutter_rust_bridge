# Overview

:::info
References in arguments (such as `fn f(a: &Foo, b: &mut Bar)`) is already supported long ago,
and is not the feature discussed here.
:::

:::info
Use `enable_lifetime: true` to enable this feature.
It is quite young and thus experimental,
therefore it may not follow the semver that the rest of this package follows,
and this part may contain bugs or missing features.
If you want to discuss anything, feel free to open an issue on GitHub.
:::

Returning types with lifetimes are supported in `flutter_rust_bridge`.
Currently, I have only implemented a subset of the features that I hope to implement,
but the rest should be usually workaround-able, and is discussed in later pages.
