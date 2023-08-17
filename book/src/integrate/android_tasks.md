# Hooking onto tasks

This is the same method used by the app template and also the easier one.
Go ahead and install `cargo-ndk` if you have not already done so:

```
cargo install cargo-ndk
```

Next, add these lines[^1] near the bottom of `android/app/build.gradle`:

```gradle
[
    new Tuple2('Debug', ''),
    new Tuple2('Profile', '--release'),
    new Tuple2('Release', '--release')
].each {
    def taskPostfix = it.first
    def profileMode = it.second
    tasks.whenTaskAdded { task ->
        if (task.name == "javaPreCompile$taskPostfix") {
            task.dependsOn "cargoBuild$taskPostfix"
        }
    }
    tasks.register("cargoBuild$taskPostfix", Exec) {
        // Until https://github.com/bbqsrc/cargo-ndk/pull/13 is merged,
        // this workaround is necessary.

        def ndk_command = """cargo ndk \
            -t armeabi-v7a -t arm64-v8a -t x86_64 -t x86 \
            -o ../android/app/src/main/jniLibs build $profileMode"""

        workingDir "../../$crate"
        environment "ANDROID_NDK_HOME", "$ANDROID_NDK"
        if (org.gradle.nativeplatform.platform.internal.DefaultNativePlatform.currentOperatingSystem.isWindows()) {
            commandLine 'cmd', '/C', ndk_command
        } else {
            commandLine 'sh', '-c', ndk_command
        }
    }
}
```

Note the ANDROID_NDK variable, this is a Gradle property that points to
your installation of the Android NDK. If you don't rely on portability,
you can hardcode this value, but note that it can be supplied by one
of the many `gradle.properties` scattered throughout your filesystem.
The most reliable way is to create a file at `~/.gradle/gradle.properties`
and fill it with this:

```
ANDROID_NDK=(path to NDK)
```

Note the ABIs `x86_64` and `x86` in `ndk_command` are usually used for Android simulators. Feel free to remove them as needed.

[^1]:
    This excerpt might be outdated, please check out
    the [source file](https://github.com/Desdaemon/flutter_rust_bridge_template/blob/main/android/app/build.gradle) at the [template repository](https://github.com/Desdaemon/flutter_rust_bridge_template).
