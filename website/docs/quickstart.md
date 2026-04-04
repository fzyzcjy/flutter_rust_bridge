import Tabs from '@theme/Tabs';
import TabItem from '@theme/TabItem';

# Quickstart

:::info

If you like to setup in one command:

```shell
cargo install flutter_rust_bridge_codegen && flutter_rust_bridge_codegen create my_app && cd my_app && flutter run
```

:::

## 1. Install

After [Flutter](https://docs.flutter.dev/get-started/install) and [Rust](https://www.rust-lang.org/tools/install) are
installed,
install `flutter_rust_bridge` using any method:

<Tabs>

<TabItem value="Default">

```shell
cargo install flutter_rust_bridge_codegen
```

</TabItem>

<TabItem value="Cargo-Binstall">

```shell
cargo binstall flutter_rust_bridge_codegen
```

</TabItem>

<!--

<TabItem value="Scoop">

<small>Remark: Thanks @Desdaemon for scripts to publish to brew/scoop</small>

```shell
scoop bucket add frb https://github.com/Desdaemon/scoop-repo
scoop install flutter_rust_bridge_codegen
```

</TabItem>

<TabItem value="Homebrew">

<small>Remark: Thanks @Desdaemon for scripts to publish to brew/scoop</small>

```shell
brew install desdaemon/repo/flutter_rust_bridge_codegen
```

</TabItem>

-->

</Tabs>

## 2. Create new projects / Add to existing projects

<Tabs>

<TabItem value="Create new">

Suppose your app is to be named `my_app`, then execute this. <small>(Use `--help` to see more options, e.g. package
name)</small>

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

Please refer to [this page](guides/miscellaneous/pure-dart).

</TabItem>

<TabItem value="Flutter Package">

Execute the following command to create a shareable Flutter package that can be used across multiple Flutter applictions:

```shell
flutter_rust_bridge_codegen create my_package --template plugin
```

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

P.S. The `build-web` command: Because Flutter Web does not have a build hook yet
([corresponding issue](https://github.com/flutter/flutter/issues/138992)).

```shell
flutter_rust_bridge_codegen build-web
# ... or any other standard Flutter ways
flutter run --web-header=Cross-Origin-Opener-Policy=same-origin --web-header=Cross-Origin-Embedder-Policy=require-corp
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

## What's next

On one hand, if you like to see a live demo, please visit [the next page](demo).

On the other hand, [the guides chapter](guides) introduces all features, customizations, common scenario how-tos, etc.
There are a lot of documentations, but there is no need to learn all in details. Instead:

import Intuition from './snippets/_tip-use-intuition.mdx';

<Intuition/>
