# Safety and CI

## CI

We heavily use CI to ensure everything is going well, including but not limited to memory safety.
Shortly speaking, we deploy not only `Valgrind` but also sanitizers (`ASAN`, `MSAN`, `LSAN`) in the CI,
which are commonly used tools to check safety issues.

As for all things that CI checks,
please refer to [the CI configuration](https://github.com/fzyzcjy/flutter_rust_bridge/tree/master/.github/workflows)
for full details,
and here is a brief list:

* Use Valgrind to check safety
* Use Sanitizers (ASAN, MSAN, LSAN) to check safety
* Testing (on Android, iOS, Windows, MacOS, Linux, Web)
* Run performance benchmarks
* Run `flutter_rust_bridge_codegen`
* Linters and code formatters
* Post-release tests (check the released binary)
* Test steps mentioned in quickstart
* ...

## Usage

This library seems to be used by (I want to say "is used by" but I need to be humble ;) ) many people
(as can be seen from [pub.dev popularity](https://pub.dev/packages/flutter_rust_bridge), blogs on the Internet, etc).
Therefore, bugs should be usually easily spotted and raised in the issue tracker.
However, during the past two years, there have not been issues about weird memory issues, hitting undefined behavior, etc.

I also personally use flutter_rust_bridge heavily in my Flutter project,
which is in production and it works quite well.
If I observe any problems, I will surely fix it in this library,
but again I have not seen any safety-related issues.

## Safety of unsafe

It is inevitable to write unsafe code, as long as we want to use Rust with another language.
The thing we can do and have done is,
make the unsafe blocks carefully limited & encapsulated,
make the code clear and well designed, use strong checkers in CI, etc.
I am happy to see that, nobody reported bugs related to this!

The vast majority of the code are written in safe Rust and (safe) Dart.
The `unsafe` code mainly happens when we need to leak a Rust Vec into a raw pointer,
and later assemble it back, in order to pass it to the Dart side.
This is widely used pattern - there are official Rust doc, and answers on StackOverflow about this also have high votes.

In terms of safety, there are two categories of codecs, and you can freely choose whichever you like.
One category uses bare minimal unsafe code, and the other maximizes performance.
Below, we only discuss the latter - since it is the harder case.

The unsafe logic is made as separated as possible with other safe logic, with as clear semantics as possible.
For example, instead of combining all logic into a single bigger `RustOpaque` Dart class,
I choose to extract a `RustArc` Dart class, which encapsulates the Rust `Arc` inside it.
Then, high-level `RustOpaque` logic will be safe as long as it uses `RustArc`'s public API,
and at the same time, `RustArc` is easy to audit because it has a clear semantics.

As another example, without extra work, `DartOpaque` is `!Send` and `!Sync`,
because the underlying objects really cannot be used in other threads.
But we `unsafe impl Send/Sync`, because we forbid users from touching or dropping it in other threads.
In V1, most logic were put inside the single `DartOpaque` class, mixing things with this unsafe part.
In V2, this unsafe logic is extracted to `GuardedBox`, and the `DartOpaque` becomes pure safe code.

These can be further seen in details in [the contributor guides](../contributing).

In addition, different parts are isolated. For example, if your project does not use opaque types,
then surely any code related to them will not be used at all.
