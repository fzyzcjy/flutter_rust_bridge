---
name: frb-android-emulator-prepare
description: Use when preparing, installing, diagnosing, or explaining the host Android Emulator environment for flutter_rust_bridge local runtime validation, including Android SDK command-line tools, emulator packages, AVD creation, host ADB server setup, and Docker-to-host ADB connectivity.
---

# FRB Android Emulator Preparation

Use this skill when a task requires a **host Android Emulator** for FRB local runtime validation. Do not use this skill for a physical Android phone that is already visible to `adb`; that path usually only needs host `adb` plus `frb-dev-env docker exec --android-host-adb`.

## Boundary

FRB Android runtime validation has two separate host paths:

- **Physical Android device**: host needs `adb`; Docker runs FRB/Flutter commands through the host ADB server.
- **Android Emulator**: host needs Java, Android SDK command-line tools, `emulator`, a system image, and an AVD; Docker still runs FRB/Flutter commands through the host ADB server.

Do not install Android Emulator packages unless the user has explicitly asked to prepare or run an emulator. Installing the emulator changes host state and downloads large SDK packages.

## Default Host Locations

Use standard Android Studio locations on macOS:

```text
~/Library/Android/sdk
~/.android/avd
~/.android
```

Respect `ANDROID_HOME` if it is already set. Otherwise use `~/Library/Android/sdk`. Do not invent repo-specific defaults such as `/workspace`, `/tmp`, or a user's personal artifacts directory.

## Preflight

Official references for this workflow:

- Android Studio and command-line tools download page: https://developer.android.com/studio
- `sdkmanager` command-line package manager: https://developer.android.com/tools/sdkmanager
- `avdmanager` command-line AVD manager: https://developer.android.com/tools/avdmanager
- Android Emulator command line: https://developer.android.com/studio/run/emulator-commandline
- ADB command-line tool: https://developer.android.com/tools/adb

Run these before installing anything:

```bash
sw_vers
uname -m
which adb || true
adb version || true
echo "${ANDROID_HOME:-}"
ls -ld "$HOME/Library/Android/sdk" "$HOME/.android" "$HOME/.android/avd" 2>/dev/null || true
which java || true
java -version || true
```

If a physical phone is enough for the task and `adb devices -l` shows it, do not prepare an emulator.

If the task requires an emulator, record whether these tools already exist:

```bash
ls "$HOME/Library/Android/sdk/cmdline-tools/latest/bin/sdkmanager" 2>/dev/null || true
ls "$HOME/Library/Android/sdk/cmdline-tools/latest/bin/avdmanager" 2>/dev/null || true
ls "$HOME/Library/Android/sdk/emulator/emulator" 2>/dev/null || true
```

## Installation Policy

Before installing or downloading anything, state exactly what will be installed and ask for approval through the command escalation mechanism.

For a minimal macOS emulator setup, expect these host changes:

```text
Homebrew openjdk, if Java is missing
~/Library/Android/sdk/cmdline-tools/latest
~/Library/Android/sdk/emulator
~/Library/Android/sdk/platform-tools
~/Library/Android/sdk/platforms/android-<api>
~/Library/Android/sdk/system-images/android-<api>/google_apis/arm64-v8a
~/.android/avd/<avd-name>.avd
~/.android/avd/<avd-name>.ini
```

Prefer ARM64 system images on Apple Silicon:

```text
system-images;android-35;google_apis;arm64-v8a
```

Use a different API level only when the task or Flutter version requires it.

## Install Java

If `/usr/bin/java` is missing and Homebrew is available, prefer the Homebrew `openjdk` formula because it does not require the sudo package installer:

```bash
HOMEBREW_NO_AUTO_UPDATE=1 brew install openjdk
```

For SDK commands in the current shell, prepend the Homebrew JDK:

```bash
export PATH="/opt/homebrew/opt/openjdk/bin:$PATH"
```

Do not use `brew install --cask temurin` in non-interactive agent runs unless the user has approved sudo-capable installation; it may invoke macOS Installer through `sudo`.

## Install Command-Line Tools

If Android Studio is not installed and `cmdline-tools/latest` is missing, install the official Android command-line tools into the standard SDK root. This follows the Android Developers command-line tools instructions: download the command-line tools package from the Android Studio page, extract it, and place it under `cmdline-tools/latest` in the Android SDK root.

```bash
mkdir -p "$HOME/Library/Android/sdk/cmdline-tools/latest"
curl -L -o /private/tmp/commandlinetools-mac-latest.zip \
  https://dl.google.com/android/repository/commandlinetools-mac-14742923_latest.zip
mkdir -p /private/tmp/frb-android-commandlinetools-extract
ditto -x -k /private/tmp/commandlinetools-mac-latest.zip /private/tmp/frb-android-commandlinetools-extract
cp -R /private/tmp/frb-android-commandlinetools-extract/cmdline-tools/. \
  "$HOME/Library/Android/sdk/cmdline-tools/latest/"
```

Then set:

```bash
export ANDROID_HOME="$HOME/Library/Android/sdk"
export ANDROID_SDK_ROOT="$ANDROID_HOME"
export PATH="/opt/homebrew/opt/openjdk/bin:$ANDROID_HOME/cmdline-tools/latest/bin:$ANDROID_HOME/platform-tools:$ANDROID_HOME/emulator:$PATH"
```

## Install Emulator Packages

Accept licenses and install the minimal packages with `sdkmanager`, the Android SDK package manager documented by Android Developers:

```bash
yes | sdkmanager --licenses
sdkmanager \
  "emulator" \
  "platform-tools" \
  "platforms;android-35" \
  "system-images;android-35;google_apis;arm64-v8a"
```

If the user wants a smaller or non-Google image, choose a matching `system-images;android-<api>;default;arm64-v8a` package instead.

## Create AVD

Use a predictable AVD name that the `frb-dev-env` examples can reference. This uses `avdmanager`, the Android Developers command-line tool for creating and managing AVDs:

```bash
avdmanager create avd \
  --name FRB_API_35 \
  --package "system-images;android-35;google_apis;arm64-v8a" \
  --device "pixel_8"
```

If the device profile is unavailable, list device profiles and choose a common phone:

```bash
avdmanager list device
```

## Start And Verify Emulator

Start the host emulator through `frb-dev-env`. The underlying emulator invocation follows the Android Emulator command-line interface:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py android emulator --avd FRB_API_35 --port 5554
```

In another terminal, start the host ADB server for Docker clients:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py android adb-server
```

Verify host ADB can see the emulator. This uses ADB, the Android Developers command-line tool for communicating with Android devices and emulators:

```bash
adb devices -l
```

Verify Docker can see the same emulator:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py docker exec --android-host-adb -- adb devices -l
```

Only after both ADB checks pass, run an FRB Android runtime command:

```bash
.claude/skills/frb-dev-env/frb_dev_env.py docker exec --android-host-adb -- \
  bash -lc './frb_internal test-flutter-native --package frb_example--flutter_via_create --flutter-test-args "--device-id emulator-5554"'
```

## Common Failures

- `adb ... could not install *smartsocket* listener: Operation not permitted`: the sandbox may block starting a host ADB server. Retry the ADB command with approved host execution, or run it manually in a normal terminal.
- `Android SDK root does not exist`: install Android Studio or command-line tools into `~/Library/Android/sdk`, or set `ANDROID_HOME`.
- `emulator: command not found`: install the SDK `emulator` package and ensure `~/Library/Android/sdk/emulator` is on `PATH`.
- No AVD found: create `FRB_API_35` with `avdmanager create avd`, or pass the existing AVD name to `android emulator --avd`.
- Docker cannot see host ADB: ensure host ADB server was started with `adb -a -L tcp:0.0.0.0:5037 server nodaemon`, then verify `host.docker.internal:5037` from the container.

## Cleanup

Stop the emulator from host ADB:

```bash
adb -s emulator-5554 emu kill
```

Stop the foreground `android adb-server` command with `Ctrl-C`. Avoid deleting SDK packages or AVDs unless the user explicitly asks.
