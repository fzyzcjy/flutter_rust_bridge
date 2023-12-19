import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

# Quickstart

:::info

If you like one-liner:

```shell
cargo install flutter_rust_bridge_codegen && flutter_rust_bridge_codegen create my_app && cd my_app && flutter run
```

:::

## 1. Install

After [Flutter](https://docs.flutter.dev/get-started/install) and [Rust](https://www.rust-lang.org/tools/install) are installed,
install `flutter_rust_bridge` using any method:

<Tabs>

<TabItem value="Default">

```shell
cargo install 'flutter_rust_bridge_codegen@^2.0.0-dev.0'
```

</TabItem>

<TabItem value="Cargo-Binstall">

```shell
cargo binstall 'flutter_rust_bridge_codegen@^2.0.0-dev.0'
```

</TabItem>

<TabItem value="Scoop">

:::caution
It installs v1 instead of v2.0.0-dev.x currently, so please wait a bit until the v2.0.0 stable is released :)
:::

<small>Remark: Thanks @Desdaemon for scripts to publish to brew/scoop</small>

```shell
scoop bucket add frb https://github.com/Desdaemon/scoop-repo
scoop install flutter_rust_bridge_codegen
```

</TabItem>

<TabItem value="Homebrew">

:::caution
It installs v1 instead of v2.0.0-dev.x currently, so please wait a bit until the v2.0.0 stable is released :)
:::

<small>Remark: Thanks @Desdaemon for scripts to publish to brew/scoop</small>

```shell
brew install desdaemon/repo/flutter_rust_bridge_codegen
```

</TabItem>

</Tabs>

## 2. Create new projects / Add to existing projects

<Tabs>

<TabItem value="Create new">

Suppose your app is to be named `my_app`, then execute:

```shell
flutter_rust_bridge_codegen create my_app
```

</TabItem>

<TabItem value="Add to existing">

Execute the following in the root folder of your Flutter project:

```shell
flutter_rust_bridge_codegen integrate
```

</TabItem>

<TabItem value="Pure dart">

If you want to use Rust with Dart (without Flutter),
[here](https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/frb_example/pure_dart)
is a working example, and
[here](https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/frb_example/dart_minimal)
is a minimal sample.
Indeed, the former contains many tests of this library, and is constantly executed in 
[the CI](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/.github/workflows/ci.yaml).
Alternatively, [this tutorial](manual/miscellaneous/archived/tutorial-pure-dart) may be helpful.

</TabItem>

<TabItem value="More approaches">

Please visit [this page](manual/integrate/overview).

</TabItem>

</Tabs>

### Remark: Directory structure

import DirectoryStructureBriefExplain from './snippets/_directory-structure-brief-explain.mdx';
import DirectoryStructureBriefList from './snippets/_directory-structure-brief-list.mdx';

<DirectoryStructureBriefExplain/>

For more details, please see [this page](guides/miscellaneous/directory).

## 3. Run it

<Tabs>

<TabItem value="Default">

Use your favorite method to run the app ([Flutter official doc](https://docs.flutter.dev/get-started/test-drive)),
as if it is just a *normal* Flutter project.
For example:

```shell
flutter run
```

</TabItem>

<TabItem value="Web">

1. The `build-web` command: Because Flutter Web does not have a build hook yet
(vote [the corresponding issue](https://github.com/flutter/flutter/issues/138992) to 
[prioritize](https://github.com/flutter/flutter/wiki/Issue-hygiene#do-not-add-me-too-or-same-or-is-there-an-update-comments-to-bugs)
it if you want).
2. Before Flutter >=3.17 release, [a hack](manual/miscellaneous/web-cross-origin) is needed.

```shell
flutter_rust_bridge_codegen build-web
flutter run # or any other standard Flutter ways
```

</TabItem>

</Tabs>

Then, you will see a greeting from Rust, displayed in Flutter (Dart).

## 4. Modify it

Suppose we add a *super*-simple Rust function in `rust/src/api/simple.rs`
(see [next chapter](guides) for all features):

```rust
pub fn hello(a: String) -> String { a.repeat(2) }
```

With all glue code automatically generated, we can call it in Dart (`lib/main.dart`):

```dart
var result = await hello(a: "Hi");
```

<details>
<summary>Explain the Dart code</summary>

* The `await` is for asynchronous code, a very frequently used feature in Dart.
* To display the result on the screen, a bit of standard Flutter knowledge may be needed.
See the existing code for an example how a String can be shown.

</details>

We need to execute the code generator whenever the Rust code is changed,
or use `--watch` to automatically re-generate when code changes:

```shell
flutter_rust_bridge_codegen generate --watch
```

If you need to install LLVM,
which is required by the popular Google official package `ffigen`,
please refer to [this doc](manual/ffigen-troubleshooting).

## What's next

import Intuition from './snippets/_tip-use-intuition.mdx';

<Intuition/>

[The guides chapter](guides) introduces all features, customizations, common scenario how-tos, etc.
