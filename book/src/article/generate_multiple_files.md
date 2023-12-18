# Generating multiple files

> Author: @dbsxdbsx

This article describes some thoughts and implementations about the feature of generating multiple files.

Before, like the [pure_dart's api.rs](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/pure_dart/rust/src/api.rs), all Apis are exposed together in a single file(block). This is not bad when the whole project is simple. But it would become quite hard to maintain or develop, when the project becomes more and more complex, especially when it is a team project. Therefore, it is time to reconstruct the code --- classify the exposed Api into proper blocks(files).

(Before going on reading, make sure that you are quite familiar with how to use [template](https://github.com/Desdaemon/flutter_rust_bridge_template) to generate code with flutter_rust_bridge. If not, take a look at the former chapters or [the basic example](https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/frb_example/pure_dart) again, please.)

## Try to classify Api into different blocks(files)

Suppose, you only have two Api in `api.rs` originally, like this:

```rust,noplayground
#![allow(unused_variables)]

pub fn simple_add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn simple_minus(a: i32, b: i32) -> i32 {
    a - b
}
```

Now you want to classify these 2 Api into 2 blocks for some reason-- say, you put the `simple_add` Api into file `api_block_1.rs` and the other into `api_block_2.rs`. And then make a little modification in `lib.rs`:

```rust,noplayground
mod api_block_1;
mod api_block_2;
```

Ok, now the question is how to deal with them with flutter_rust_bridge. From the [template justfile](https://github.com/Desdaemon/flutter_rust_bridge_template/blob/main/justfile#L11), we know code from a single Api file called `api_rs` can be generated with a command like this:

```
gen:
    export REPO_DIR="$PWD"; cd /; flutter_rust_bridge_codegen {{llvm_path}} \
        --rust-input "$REPO_DIR/native/src/api.rs" \
        --dart-output "$REPO_DIR/lib/bridge_generated.dart" \
...
```

(For simplicity, only two necessary flags `rust-input` and `dart-output` here.)

Then, to generate code within 2 blocks(files), you may come out with an approach like this:

```
gen:
    export REPO_DIR="$PWD"; cd /; flutter_rust_bridge_codegen {{llvm_path}} \
        --rust-input "$REPO_DIR/native/src/api_block_1.rs" \
        --dart-output "$REPO_DIR/lib/bridge_generated_api_block_1.dart" \

    export REPO_DIR="$PWD"; cd /; flutter_rust_bridge_codegen {{llvm_path}} \
        --rust-input "$REPO_DIR/native/src/api_block_2.rs" \
        --dart-output "$REPO_DIR/lib/bridge_generated_api_block_2.dart" \
...
```

But here comes a problem, how to use them in Dart? Like `await Api.simpleAdd(1,2)` or
`await Api.simpleMinus(1,2)` as before? The point here is, to thoroughly decouple Api from different blocks (which is the main reason for using multiple blocks of Api), **flag `class-name` is needed**. So the command should be modified like this:

```
gen:
    export REPO_DIR="$PWD"; cd /; flutter_rust_bridge_codegen {{llvm_path}} \
        --rust-input "$REPO_DIR/native/src/api_block_1.rs" \
        --dart-output "$REPO_DIR/lib/bridge_generated_api_block_1.dart" \
        --class-name ApiBlock1Class

    export REPO_DIR="$PWD"; cd /; flutter_rust_bridge_codegen {{llvm_path}} \
        --rust-input "$REPO_DIR/native/src/api_block_2.rs" \
        --dart-output "$REPO_DIR/lib/bridge_generated_api_block_2.dart" \
        --class-name ApiBlock2Class
...
```

(The class name `ApiBlock1Class` and `ApiBlock2Class` are chosen arbitrarily here.)

So now it seems to be perfect to generate code and use Api in Dart like `ApiBlock1Class.simpleAdd(1,2)` or `ApiBlock2Class.simpleMinus(1,2)`.

But actually, the above command is still not enough to generate code correctly. Because multiple blocks need to be translated respectively through FFI. So on the Rust side, instead of generating code to a single file [`bridge_generated.rs`](https://github.com/Desdaemon/flutter_rust_bridge_template/blob/main/native/src/bridge_generated.rs), now there are 2 files needed. But, what are the names of these 2 auto-generated Rust files?
Here, for less misunderstanding, flutter_rust_bridge decides to ask for another compulsory flag `rust-output`. So the command should be modified like this:

```
gen:
    export REPO_DIR="$PWD"; cd /; flutter_rust_bridge_codegen {{llvm_path}} \
        --rust-input "$REPO_DIR/native/src/api_block_1.rs" \
        --dart-output "$REPO_DIR/lib/bridge_generated_api_block_1.dart" \
        --class-name ApiBlock1Class \
        --rust-output generated_api_block_1

    export REPO_DIR="$PWD"; cd /; flutter_rust_bridge_codegen {{llvm_path}} \
        --rust-input "$REPO_DIR/native/src/api_block_2.rs" \
        --dart-output "$REPO_DIR/lib/bridge_generated_api_block_2.dart" \
        --class-name ApiBlock2Class \
        --rust-output generated_api_block_2
...
```

(Still, the Rust output name `generated_api_block_1` and `generated_api_block_2` are chosen arbitrarily here.)

That is, flutter_rust_bridge asks you to manually define the generated Rust file names, feel free to choose any name you like.

## Some issues with separate commands
Based on the last commands we come up with, everything seems to be fine --- the code generated, you can use them in Dart, and the whole project is compilable. And you would also notice some changes in `lib.rs`:

```rust,noplayground
mod api_block_1;
mod api_block_2;
mod generated_api_block_1; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
mod generated_api_block_2; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
```

But actually, it is not good enough.

### issue from explicit Api conflict

Let's say one day, you decide to add another Api, say `simpleDivide`. But when you compile the whole project, the Dart compiler just complains "The symbol `simpleDivide` has already been defined ...". Then you check whether this `simpleDivide` is defined duplicated. Finally, you find that it's already defined in another block. This situation occurs quite often, when the other block is in charge of someone else, especially in a big project. It is easy to see that the whole routine is a little inefficient since you don't realize the Api conflict until doing compiling when you've probably coded a lot with this "newly defined" Api --- and the more time compiling takes, the more inefficient.

### issue from implicit Api conflict

And what makes the Api conflict issue more catastrophic? Say you define another Api with parameter `String` in `api_block_1.rs`:

```rust,noplayground
pub fn test_string_1(s1: String) {
    println!("test implicit parameter conflicts {}", s1);
}
```

And then you put another Api with parameter `String` in `api_block_2.rs`:

```rust,noplayground
pub fn test_string_2(s2: String) {
    println!("test implicit parameter conflicts {}", s2);
}
```

These 2 Apis don't violate the uniqueness required by FFI. They should be compilable with no error. But the truth is no! Why? Because for the `String` parameter, flutter_rust_bridge would automatically generate Api like this:

```rust,noplayground
#[no_mangle]
pub extern "C" fn new_uint_8_list(len: i32) -> *mut wire_uint_8_list
```

which is used to let Rust code easily cooperate with Dart through FFI. So if there are 2 Apis both taking `String` as parameters over blocks, you should notice a similar panic like "the symbol `new_uint_8_list` is already defined ..." during compiling([issue #511](https://github.com/fzyzcjy/flutter_rust_bridge/issues/511)).

(Actually, since version [1.37](https://github.com/fzyzcjy/flutter_rust_bridge/releases/tag/v1.37.0), even with the separated commands with no Api defined, the whole project is still not compilable with error "symbol `free_WireSyncReturnStruct` is already defined... ", the symbol `free_WireSyncReturnStruct` is another implicitly Api generated by flutter_rust_bridge. Let alone the case with Api taking customized defined types as parameters, [issue #987](https://github.com/fzyzcjy/flutter_rust_bridge/issues/987#issuecomment-1385089679))

So these kinds of explicit/implicit Api conflicts are annoying and frustrating. How to resolve it?

Theoretically, the conflict can be detected earlier during generating code, when flutter_rust_bridge knows every detail about Api. But the key is that **flutter_rust_bridge has to know all Api over all blocks before generating code**. That is, with the separated command stated above, flutter_rust_bridge can't do the check for you in practice. Therefore, it is necessary to unite the separated commands into ONE command.

## Correct command for generating code with multiple blocks

Now comes the joined command to resolve the above issue:

```
gen:
    export REPO_DIR="$PWD"; cd /; flutter_rust_bridge_codegen {{llvm_path}} \
        --rust-input "$REPO_DIR/native/src/api_block_1.rs" "$REPO_DIR/native/src/api_block_2.rs" \
        --dart-output "$REPO_DIR/lib/bridge_generated_api_block_1.dart" "$REPO_DIR/lib/bridge_generated_api_block_2.dart" \
        --class-name ApiBlock1Class ApiBlock2Class \
        --rust-output generated_api_block_1 generated_api_block_2
...
```

Here, with just 1 command, flutter_rust_bridge would smartly check if there are conflicts over all Api over all blocks, be it defined explicitly or implicitly.

That is, for the explicitly defined Apis like `simple_add` and `simple_minus`, if there are duplicated ones, flutter_rust_bridge would throw a panic like "thread 'main' panicked at 'symbol [simple_add] has already been defined'...", and you are responsible to fix it. And for the implicitly defined Api like `new_uint_8_list`, since it is essentially used for multiple blocks, flutter_rust_bridge would define it in a new auto-generated file called `bridge_generated_shares.rs` as default, whatever the type is shared as input, output or both among Apis. So you don't need to worry about it(Note: in the [multi_block example](https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/frb_example/pure_dart_multi/rust/build.rs) example, `new_uint_8_list` is actually defined in `bridge_generated_shares.io.rs` since flag `--wasm` is used).

To sum up, **there are 4 compulsory flags when you deal with multiple blocks.** They are `rust-input`, `dart-output`, `class-name`, and `rust-output`. Also, the number of fields following each flag should be consistent. You can try to `cargo build` with fewer flags or inconsistent fields to see what kind of panic would be popped up with the [multi_block example](https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/frb_example/pure_dart_multi/rust/build.rs) when doing generation.

## bizarre, weird but compilable command with the disorder

**Flutter_rust_bridge doesn't do semantic correction over all flags.** So, it is syntactically correct with the following generation command:

```
gen:
    export REPO_DIR="$PWD"; cd /; flutter_rust_bridge_codegen {{llvm_path}} \
        --rust-input "$REPO_DIR/native/src/api_orange.rs" "$REPO_DIR/native/src/api_apple.rs" \
        --dart-output "$REPO_DIR/lib/gen_api_apple.dart" "$REPO_DIR/lib/gen_api_orange.dart" \
        --class-name ApiClassOrange ApiClassApple \
        --rust-output generated_api_apple generated_api_orange
```

NOTE: the suffix `apple` and `orange` are quite disordered for each flag here on purpose. It is compilable and usable. But as you should know, it is not a good practice, semantically. It is all up to you to decide the field names for each flag, so beware of it!

## More about the auto-generated shared block

### Shared Rust files

As mentioned above, flutter_rust_bridge would automatically generate a new file called `bridge_generated_shares.rs` for the shared types --- To make it clear and short, let's call all the related generated stuff(the Rust files, Dart files, and C files) the shared (Api) block; and on the contrary, those normally generated stuff is called regular (Api) block.

In a shared block, there is stuff including any shared types like primitive, customized defined types(structs, enums)list(vector), even the type is wrapped with `option`, or `SyncReturn`. And note: For a shared customized type, please DON'T define it in any regular Api block, since it is both not a good practice semantically and also lead to unexplainable issue with code generation(It is planed to throw an error when this happens in future version).
Note: It is NOT ok to define a shared customized type in any regular API block, since it is not a good practice semantically and can lead to unexplainable issues with code generation in flutter_rust_bridge. In future versions, an error will be thrown if this happens.

Currently, for the generated name of the shared block, flutter_rust_bridge would automatically name it `bridge_generated_shares.rs` as default. But you can also customize it with flag `shared_rust_output` like `---shared_rust_output my_shared.rs"`, `--shared_rust_output ./another_shared_name.rs`. **Be aware that the directory is not supported**---For a shared_rust_output field like `custom_directory/my_shared.rs`, it would be treated as `./my_shared.rs`. The reason is that it is hard to handle the generation logic for the shared block in a directory that is not the same as those of the regular blocks. And yes, when there are multiple regular blocks, please put ALL of them in the SAME directory---That is, make sure `rust_output` and `shared_rust_output` don't contain any path string.

Note: For a shared used customized type(struct/enum), DON'T define it in ANY regular Api definition file(referred by flag `rust-input`). Because it is not a good practice, also it would generate issued Rust files.


### Shared Dart file

For the generated Dart files, all the generated stuff is just like the Rust files. And to use a customized type that is shared, just like for the regular one, the shared Api object needs to be defined, like this:
```dart
// This file initializes the dynamic library and connects it with the stub
// generated by flutter_rust_bridge_codegen.

import 'dart:ffi';
import 'dart:io' as io;

import 'gened_api_1.dart';
import 'gened_api_2.dart';

const _base = 'native';

// On MacOS, the dynamic library is not bundled with the binary,
// but rather directly **linked** against the binary.
final _dylib = io.Platform.isWindows ? '$_base.dll' : 'lib$_base.so';

final api1 = Api1Impl(io.Platform.isIOS || io.Platform.isMacOS
    ? DynamicLibrary.executable()
    : DynamicLibrary.open(_dylib));

final api2 = Api2Impl(io.Platform.isIOS || io.Platform.isMacOS
    ? DynamicLibrary.executable()
    : DynamicLibrary.open(_dylib));

final apiShare = BridgeGeneratedSharesImpl(
    io.Platform.isIOS || io.Platform.isMacOS
        ? DynamicLibrary.executable()
        : DynamicLibrary.open(_dylib));

```
Here, `apiShare` is responsible for any shared types. For example, if there is a shared type named `SharedStructInBlock1And2`, now it can be used with `bridge` as the shared Api object, like this:
```dart
    var obj = SharedStructInBlock1And2(bridge: apiShared, name: "newString", id: 2, num: 2.2);
    // using methods
    expect("msg_1", await obj.testMethod(message: "msg", num: 1));
    // using static methods directly by class name
    expect("msg", await SharedStructInBlock1And2.testStaticMethod(bridge: api1, message: "msg"));
```

### Shared C header file

Besides, for the generated C (header) files, like the corresponding generated Rust and Dart files, there would be also an auto-generated C file with the name `bridge_generated_shared.h`(the name would also be changed by flag `shared_rust_output`) besides the other C files for regular blocks.

This shared C file, besides all shared types and Apis, also includes ALL the headers of all regular C files. That is, it could be treated as the manager of all C files.

Generally, the C files are transparent to users, since they are generated in temporary folders as default(when not using `c-output-path` and `extra-c-output-path`). But if the platform is iOS/MacOS, then it is inevitable to take care of them(For more details with iOS/MacOS, please refer to the section "For multi-blocks" in [ios_headers](https://cjycode.com/flutter_rust_bridge/integrate/ios_headers.html#for-multi-blocks)).


### Something not implemented

Currently, those complex types like `stream`, and `opaque`, are not yet supported to be used as shared types. For `enum`, it is supported to be shared, but the methods of this kind of type are not supported to be generated in Dart yet.

Besides, the method within a shared type may be incompatible with flags `extra-headers` and `no-use-bridge-in-method`.