# Flamegraph

:::caution
Normal users do not need this page.
Only read this page if you are interested in looking into the performance of your use case,
or want to contribute to further improve this package's performance.
:::

This page contains a laundry list to get a flamegraph on my macOS.
For other operating systems or scenarios, the detailed commands may be a bit different.

References: 

* How to symbolicate Dart things: https://github.com/dart-lang/sdk/issues/54207
* Do profiling on MacOS with SIP enabled: https://poweruser.blog/using-dtrace-with-sip-enabled-3826a352e64b

```shell
# prepare `dartaotruntime`
cp /Users/tom/fvm/default/bin/cache/dart-sdk/bin/dartaotruntime ~/temp/dartaotruntime
sudo codesign --remove ~/temp/dartaotruntime

# compile
export DART_SDK=/Users/tom/fvm/default/bin/cache/dart-sdk
$DART_SDK/bin/dartaotruntime $DART_SDK/bin/snapshots/gen_kernel_aot.dart.snapshot --platform=$DART_SDK/lib/_internal/vm_platform_strong.dill --aot --tfa -o build/simple_benchmark.dill benchmark/simple_benchmark.dart
$DART_SDK/bin/utils/gen_snapshot --snapshot-kind=app-aot-assembly --assembly=build/simple_benchmark.S build/simple_benchmark.dill
gcc -shared -o build/simple_benchmark.so build/simple_benchmark.S

# run flamegraph (example)
sudo flamegraph -o build/my_flamegraph.svg -- ~/temp/dartaotruntime build/simple_benchmark.so loop build/whatever.out whatever 'VoidFunction.*Frb.*false' 10000000
```
