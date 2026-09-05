import 'dart:io';

import 'package:code_assets/code_assets.dart';
import 'package:flutter_rust_bridge_hooks/flutter_rust_bridge_hooks.dart';
import 'package:native_toolchain_rust/native_toolchain_rust.dart'
    as native_toolchain_rust;
import 'package:test/test.dart';

void main() {
  test(
    'builder defaults to generated IO asset, rust crate path, and release build mode',
    () {
      const builder = FlutterRustBridgeNativeAssetsBuilder();

      expect(builder.assetName, 'src/rust/frb_generated.io.dart');
      expect(builder.cratePath, 'rust');
      expect(builder.buildMode, native_toolchain_rust.BuildMode.release);
    },
  );

  test('builder keeps explicit build mode override', () {
    const builder = FlutterRustBridgeNativeAssetsBuilder(
      buildMode: native_toolchain_rust.BuildMode.debug,
    );

    expect(builder.buildMode, native_toolchain_rust.BuildMode.debug);
  });

  group('cargoEnvironmentWithAndroidPageSize', () {
    test('wraps the Android arm64 linker without changing rustflags', () async {
      final outputDirectory = await _createTemporaryOutputDirectory();
      const extraCargoEnvironmentVariables = {
        'RUSTFLAGS': '-C opt-level=2',
        'CARGO_TARGET_AARCH64_LINUX_ANDROID_RUSTFLAGS': '-C debuginfo=1',
      };

      final environment = await cargoEnvironmentWithAndroidPageSize(
        targetOS: OS.android,
        targetArchitecture: Architecture.arm64,
        compiler: Uri.file('/android/ndk/toolchains/llvm/bin/clang'),
        outputDirectory: outputDirectory.uri,
        isWindows: true,
        extraCargoEnvironmentVariables: extraCargoEnvironmentVariables,
      );
      final wrapper = File(
        environment['CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER']!,
      );

      expect(environment, {
        ...extraCargoEnvironmentVariables,
        'CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER': wrapper.path,
      });
      expect(
        await wrapper.readAsString(),
        contains('aarch64-linux-android35-clang.cmd'),
      );
      expect(
        await wrapper.readAsString(),
        contains(
          '-Wl,-z,max-page-size=16384 '
          '-Wl,-z,common-page-size=16384',
        ),
      );
    });

    test('wraps an explicit Android x64 linker', () async {
      final outputDirectory = await _createTemporaryOutputDirectory();
      const linkerEnvironmentVariable =
          'CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER';
      const extraCargoEnvironmentVariables = {
        linkerEnvironmentVariable: '/custom/android-linker',
        'CARGO_ENCODED_RUSTFLAGS': '-C\x1fopt-level=2',
      };

      final environment = await cargoEnvironmentWithAndroidPageSize(
        targetOS: OS.android,
        targetArchitecture: Architecture.x64,
        compiler: Uri.file('/android/ndk/toolchains/llvm/bin/clang'),
        outputDirectory: outputDirectory.uri,
        isWindows: true,
        extraCargoEnvironmentVariables: extraCargoEnvironmentVariables,
      );

      expect(environment['CARGO_ENCODED_RUSTFLAGS'], '-C\x1fopt-level=2');
      expect(
        await File(environment[linkerEnvironmentVariable]!).readAsString(),
        contains('"/custom/android-linker" %*'),
      );
    });

    test('writes an executable POSIX linker wrapper', () async {
      if (Platform.isWindows) {
        return;
      }

      final outputDirectory = await _createTemporaryOutputDirectory();
      const linkerEnvironmentVariable =
          'CARGO_TARGET_X86_64_LINUX_ANDROID_LINKER';
      final environment = await cargoEnvironmentWithAndroidPageSize(
        targetOS: OS.android,
        targetArchitecture: Architecture.x64,
        compiler: Uri.file('/android/ndk/toolchains/llvm/bin/clang'),
        outputDirectory: outputDirectory.uri,
        isWindows: false,
        extraCargoEnvironmentVariables: const {
          linkerEnvironmentVariable: '/bin/echo',
        },
      );
      final wrapper = File(environment[linkerEnvironmentVariable]!);
      final result = await Process.run(wrapper.path, ['input.so']);

      expect(result.exitCode, 0);
      expect(
        (result.stdout as String).trim(),
        'input.so -Wl,-z,max-page-size=16384 '
        '-Wl,-z,common-page-size=16384',
      );
    });

    test('requires a compiler for Android 64-bit targets', () async {
      final outputDirectory = await _createTemporaryOutputDirectory();

      expect(
        () => cargoEnvironmentWithAndroidPageSize(
          targetOS: OS.android,
          targetArchitecture: Architecture.arm64,
          compiler: null,
          outputDirectory: outputDirectory.uri,
          isWindows: false,
          extraCargoEnvironmentVariables: const {},
        ),
        throwsA(isA<UnsupportedError>()),
      );
    });

    test('keeps non-Android targets unchanged', () async {
      const extraCargoEnvironmentVariables = {'RUSTFLAGS': '-C opt-level=2'};

      expect(
        await cargoEnvironmentWithAndroidPageSize(
          targetOS: OS.linux,
          targetArchitecture: Architecture.x64,
          compiler: null,
          outputDirectory: Uri.directory('/unused/'),
          isWindows: false,
          extraCargoEnvironmentVariables: extraCargoEnvironmentVariables,
        ),
        same(extraCargoEnvironmentVariables),
      );
    });

    test('keeps Android 32-bit targets unchanged', () async {
      const extraCargoEnvironmentVariables = {'RUSTFLAGS': '-C opt-level=2'};

      expect(
        await cargoEnvironmentWithAndroidPageSize(
          targetOS: OS.android,
          targetArchitecture: Architecture.arm,
          compiler: null,
          outputDirectory: Uri.directory('/unused/'),
          isWindows: false,
          extraCargoEnvironmentVariables: extraCargoEnvironmentVariables,
        ),
        same(extraCargoEnvironmentVariables),
      );
    });
  });

  test('buildInputForHost uses short Windows output directory', () async {
    final input = _createBuildInput(outputDirectoryShared: '/tmp/frb-long');
    final adjusted = await buildInputForHost(isWindows: true, input: input);
    final shortOutputDirectoryShared = adjusted.outputDirectoryShared;
    final shortOutputDirectorySharedPath = Directory.fromUri(
      shortOutputDirectoryShared,
    ).path;

    expect(
      shortOutputDirectorySharedPath,
      startsWith(
        '${Directory.systemTemp.path}${Platform.pathSeparator}'
        'frb_native_assets_',
      ),
    );
    expect(shortOutputDirectoryShared, isNot(input.outputDirectoryShared));
    expect(adjusted.json, {
      ...input.json,
      'out_dir_shared': shortOutputDirectorySharedPath,
    });
    expect(
      Directory.fromUri(adjusted.outputDirectory).path,
      startsWith(shortOutputDirectorySharedPath),
    );
  });

  test('buildInputForHost keeps non-Windows input unchanged', () async {
    final input = _createBuildInput(outputDirectoryShared: '/tmp/frb-long');

    expect(
      await buildInputForHost(isWindows: false, input: input),
      same(input),
    );
  });
}

Future<Directory> _createTemporaryOutputDirectory() async {
  final directory = await Directory.systemTemp.createTemp(
    'frb_native_assets_builder_test_',
  );
  addTearDown(() => directory.delete(recursive: true));
  return directory;
}

BuildInput _createBuildInput({required String outputDirectoryShared}) {
  final builder = BuildInputBuilder()
    ..setupShared(
      packageRoot: Uri.directory('/tmp/frb-package/'),
      packageName: 'frb_package',
      outputDirectoryShared: Uri.directory('$outputDirectoryShared/'),
      outputFile: Uri.file('/tmp/frb-output.json'),
    )
    ..setupBuildInput();
  builder.config.setupBuild(linkingEnabled: false);
  return builder.build();
}
