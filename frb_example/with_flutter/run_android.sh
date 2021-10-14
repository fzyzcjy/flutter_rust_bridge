set -eux

# NOTE need to run cargo build and flutter build *separately*, since we have not (but *can*) integrate into one
(cd rust && cargo ndk -t armeabi-v7a build --release -vv)

mkdir -p android/app/src/main/jniLibs/armeabi-v7a
cp rust/target/armv7-linux-androideabi/release/libflutter_rust_bridge_example.so android/app/src/main/jniLibs/armeabi-v7a/libflutter_rust_bridge_example.so

flutter build apk

adb install -r build/app/outputs/flutter-apk/app-release.apk

adb shell 'am start -n com.example.frb_example/com.example.frb_example.MainActivity'