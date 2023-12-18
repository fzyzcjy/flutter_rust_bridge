# Set up Flutter/Dart+Rust support from scratch

> This documentation is archived, though technically still correct. Have a look at `integrating with existing projects` chapters for a more detailed demonstration.

I suggest that you can start with the [Flutter example](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/with_flutter) first, and modify it to satisfy your needs. It can serve as a template for new projects. It is run against CI so we are sure it works.

Indeed, this library is nothing but a code generator that helps your Flutter/Dart functions call Rust functions. Therefore, "how to create a Flutter app that can run Rust code" is actually out of the scope of this library, and there are already several tutorials on the Internet.

However, I can sketch the outline of what to do if you want to set up a new Flutter+Rust project as follows.

## Step 1

Create a new Flutter project (or use an existing one). The Dart SDK should be `>=2.17.0` if you want to use the latest `ffigen` tool.

## Step 2

Create a new Rust project, say, at directory `rust` under the Flutter project.

## Step 3

Edit `Cargo.toml` and add:

```diff
[lib]
name = "flutter_rust_bridge_example" # whatever you like
# notice this type. `cdylib` for android, and `staticlib` for iOS. I write down a script to change it before build.
+ crate-type = ["cdylib"]
```

## Step 4

Follow the standard steps of "how iOS uses static libraries".
1. In XCode, edit `Strip Style` in `Build Settings` to `Debugging Symbols`.
2. Add your `lib{crate}.a` to `Link Binary With Libraries` in `Build Phases`.
3. Add `binding.h` to `Copy Bundle Resources`.
4. Add `#import "binding.h"` to `Runner-Bridging-Header`.
5. Last but not least, add a never-to-be-executed dummy function in Swift that calls any of the generated C bindings. This lib has already generated a dummy method for you, so you simply need to add `print("dummy_value=\(dummy_method_to_enforce_bundling())");` to swift file's `override func application(...) {}`, and this will prevent symbol stripping - especially in the release build for iOS (i.e. when building ipa file or releasing to App Store). Notice that, we have to use that `dummy_method_to_enforce_bundling()`, otherwise the symbols will not maintain in the release build, and Flutter will complain it cannot find the symbols.

## Step 5

Lastly, in order to build the Rust library automatically when you are building Flutter, follow [this tutorial](https://stackoverflow.com/q/69515032/4619958).
