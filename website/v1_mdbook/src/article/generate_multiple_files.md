# Generating multiple files

> Author: @dbsxdbsx

This article describes some thoughts and implementations about the feature of generating multiple files.

Before, like the [pure_dart's api.rs](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/pure_dart/rust/src/api.rs), all APIs are exposed together in a single file(block). This is not bad when the whole project is simple. But it would become quite hard to maintain or develop, when the project becomes more and more complex, especially when it is a team project. Therefore, it is time to reconstruct code --- classify the exposed Api into proper blocks(files).

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

Now you want to classify these 2 Api into 2 blocks for some reason-- say, you put the `simple_add` Api into file `api_1.rs` and the other into `api_2.rs`. And then make a little modification in `lib.rs`:

```rust,noplayground
mod api_1;
mod api_2;
```

Ok, now the question is how to deal with them with flutter_rust_bridge? From the [template justfile](https://github.com/Desdaemon/flutter_rust_bridge_template/blob/main/justfile#L11), we know code from a single API file called `api_rs` can be generated with a command like this:

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
        --rust-input "$REPO_DIR/native/src/api_1.rs" \
        --dart-output "$REPO_DIR/lib/bridge_generated_api_1.dart" \

    export REPO_DIR="$PWD"; cd /; flutter_rust_bridge_codegen {{llvm_path}} \
        --rust-input "$REPO_DIR/native/src/api_2.rs" \
        --dart-output "$REPO_DIR/lib/bridge_generated_api_2.dart" \
...
```

But here comes a problem, how to use them in dart? Like `await API.simpleAdd(1,2)` or
`await API.simpleMinus(1,2)` as before? The point here is, to thoroughly decouple Api from different blocks (which is the main reason for using multiple blocks of API), **flag `class-name` is needed**. So the command should be modified like this:

```
gen:
    export REPO_DIR="$PWD"; cd /; flutter_rust_bridge_codegen {{llvm_path}} \
        --rust-input "$REPO_DIR/native/src/api_1.rs" \
        --dart-output "$REPO_DIR/lib/bridge_generated_api_1.dart" \
        --class-name ApiClass1

    export REPO_DIR="$PWD"; cd /; flutter_rust_bridge_codegen {{llvm_path}} \
        --rust-input "$REPO_DIR/native/src/api_2.rs" \
        --dart-output "$REPO_DIR/lib/bridge_generated_api_2.dart" \
        --class-name ApiClass2
...
```

(The class name `ApiClass1` and `ApiClass2` are chosen arbitrarily here.)

So now it seems to be perfect to generate code and using Api in Dart like `ApiClass1.simpleAdd(1,2)` or `ApiClass2.simpleMinus(1,2)`.

But actually, the above command is still not enough to generate code correctly. Because multiple blocks need to be translated respectively through FFI. So on the rust side, instead of generating code to a single file [`bridge_generated.rs`](https://github.com/Desdaemon/flutter_rust_bridge_template/blob/main/native/src/bridge_generated.rs), now there are 2 files needed. But, what are the names of these 2 auto-generated rust files?
Here, for less misunderstanding, flutter_rust_bridge decides to ask for another compulsory flag `rust-output`. So the command should be modified like this:

```
gen:
    export REPO_DIR="$PWD"; cd /; flutter_rust_bridge_codegen {{llvm_path}} \
        --rust-input "$REPO_DIR/native/src/api_1.rs" \
        --dart-output "$REPO_DIR/lib/bridge_generated_api_1.dart" \
        --class-name ApiClass1 \
        --rust-output generated_api_1

    export REPO_DIR="$PWD"; cd /; flutter_rust_bridge_codegen {{llvm_path}} \
        --rust-input "$REPO_DIR/native/src/api_2.rs" \
        --dart-output "$REPO_DIR/lib/bridge_generated_api_2.dart" \
        --class-name ApiClass2 \
        --rust-output generated_api_2
...
```

(Still, the rust output name `generated_api_1` and `generated_api_2` are chosen arbitrarily here.)

That is, flutter_rust_bridge asks you to manually define the generated rust file names, feel free to choose any name you like.

## Some issues with separate commands
Based on the last commands we come up with, everything seems to be fine --- the code generated, you can use them in Dart, and the whole project is compilable. And you would also notice some changes in `lib.rs`:

```rust,noplayground
mod api_1;
mod api_2;
mod generated_api_1; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
mod generated_api_2; /* AUTO INJECTED BY flutter_rust_bridge. This line may not be accurate, and you can change it according to your needs. */
```

But actually, it is not good enough.

### issue from explicit Api conflict

Let's say one day, you decide to add another API, say `simpleDivide`. But when you compile the whole project, the Dart compiler just complains "The symbol `simpleDivide` has already been defined ...". Then you check whether this `simpleDivide` is defined duplicated. Finally, you find that it's already defined in another block. This situation occurs quite a lot, when the other block is in the charge of someone else, especially in a big project. It is easy to see that the whole routine is a little inefficient since you don't realize the Api conflict until doing compiling when you've probably coded a lot with this "new defined" Api --- and the more time compiling takes, the more inefficient.

### issue from implicit Api conflict

And what makes the Api conflict issue more catastrophic? Say you define another Api with parameter `String` in `api_1.rs`:

```rust,noplayground
pub fn test_string_1(s1: String) {
    println!("test implicit parameter conflicts {}", s1);
}
```

And then you put another Api with parameter `String` in `api_2.rs`:

```rust,noplayground
pub fn test_string_2(s2: String) {
    println!("test implicit parameter conflicts {}", s2);
}
```

These 2 Apis don't violate the uniqueness required by FFI. They should be compilable with no error. But the truth is no! Why? Because for the `String` parameter, flutter_rust_bridge would automatically generate API like this:

```rust,noplayground
#[no_mangle]
pub extern "C" fn new_uint_8_list(len: i32) -> *mut wire_uint_8_list
```

which is used to let rust code easily cooperate with Dart through FFI. So if there are 2 APIs both taking `String` as parameters over blocks, you should notice a similar panic like "the symbol `new_uint_8_list` is already defined ..." during compiling([issue #511](https://github.com/fzyzcjy/flutter_rust_bridge/issues/511)).

(Actually, since version [1.37](https://github.com/fzyzcjy/flutter_rust_bridge/releases/tag/v1.37.0), even with the separated commands with no Api defined, the whole project is still not compilable with error "symbol `free_WireSyncRust2DartStruct` is already defined... ", the symbol `free_WireSyncRust2DartStruct` is another implicitly Api generated by flutter_rust_bridge.)

So these kinds of explicit/implicit Api conflicts are annoying and frustrating. How to resolve it?

Theoretically, the conflict can be detected earlier during generating code, when flutter_rust_bridge knows every detail about API. But the key is that **flutter_rust_bridge has to know all Api over all blocks before generating code**. That is, with the separated command stated above, flutter_rust_bridge can't do the check for you in practice. Therefore, it is necessary to unite the separated commands into ONE command.

## correct command for generating code with multiple blocks

Now comes the joined command to resolve the above issue:

```
gen:
    export REPO_DIR="$PWD"; cd /; flutter_rust_bridge_codegen {{llvm_path}} \
        --rust-input "$REPO_DIR/native/src/api_1.rs" "$REPO_DIR/native/src/api_2.rs" \
        --dart-output "$REPO_DIR/lib/bridge_generated_api_1.dart" "$REPO_DIR/lib/bridge_generated_api_2.dart" \
        --class-name ApiClass1 ApiClass2 \
        --rust-output generated_api_1 generated_api_2
...
```

Here, with just 1 command, flutter_rust_bridge would smartly check if there are conflicts over all Api over all blocks, be it defined explicitly or implicitly.

That is, for the explicitly defined APIs like `simple_add` and `simple_minus`, if there are duplicated ones, flutter_rust_bridge would throw a panic like "thread 'main' panicked at 'symbol [simple_add] has already been defined'...", and you are responsible to fix it. And for the implicitly defined API like `new_uint_8_list`, since it is essential, flutter_rust_bridge would try to work around it by adding suffix starting from 0, like `new_uint_8_list_0` and `new_uint_8_list_1`.

To sum up, **there are 4 compulsory flags when you deal with multiple blocks.** They are `rust-input`, `dart-output`, `class-name` and `rust-output`. Also, the number of fields following each flag should be consistent. You can try to `cargo build` with fewer flags or inconsistent fields to see what kind of panic would be popped up with the [pure_dart_multi](https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/frb_example/pure_dart_multi/rust/build.rs) example when doing generation.

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

NOTE: the suffix `apple` and `orange` are quite disordered for each flag here on purpose. It is compilable and usable. But as you should know, it is not a good practice, semantically. It is all up to you to decide the field names for each flag, so be beware of it!