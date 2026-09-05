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
    const pageSizeRustFlags =
        '-C link-arg=-Wl,-z,max-page-size=16384 '
        '-C link-arg=-Wl,-z,common-page-size=16384';

    test('adds target flags for Android arm64', () {
      expect(
        cargoEnvironmentWithAndroidPageSize(
          targetOS: OS.android,
          targetArchitecture: Architecture.arm64,
          parentEnvironment: const {},
          extraCargoEnvironmentVariables: const {},
        ),
        const {
          'CARGO_TARGET_AARCH64_LINUX_ANDROID_RUSTFLAGS': pageSizeRustFlags,
        },
      );
    });

    test('appends to target flags for Android x64', () {
      expect(
        cargoEnvironmentWithAndroidPageSize(
          targetOS: OS.android,
          targetArchitecture: Architecture.x64,
          parentEnvironment: const {},
          extraCargoEnvironmentVariables: const {
            'CARGO_TARGET_X86_64_LINUX_ANDROID_RUSTFLAGS': '-C opt-level=2',
          },
        ),
        const {
          'CARGO_TARGET_X86_64_LINUX_ANDROID_RUSTFLAGS':
              '-C opt-level=2 $pageSizeRustFlags',
        },
      );
    });

    test('appends to active global RUSTFLAGS', () {
      expect(
        cargoEnvironmentWithAndroidPageSize(
          targetOS: OS.android,
          targetArchitecture: Architecture.arm64,
          parentEnvironment: const {'RUSTFLAGS': '-C opt-level=2'},
          extraCargoEnvironmentVariables: const {
            'CARGO_TARGET_AARCH64_LINUX_ANDROID_RUSTFLAGS': '-C debuginfo=1',
          },
        ),
        const {
          'CARGO_TARGET_AARCH64_LINUX_ANDROID_RUSTFLAGS': '-C debuginfo=1',
          'RUSTFLAGS': '-C opt-level=2 $pageSizeRustFlags',
        },
      );
    });

    test('appends encoded flags with Cargo unit separators', () {
      expect(
        cargoEnvironmentWithAndroidPageSize(
          targetOS: OS.android,
          targetArchitecture: Architecture.x64,
          parentEnvironment: const {
            'CARGO_ENCODED_RUSTFLAGS': '-C\x1fopt-level=2',
          },
          extraCargoEnvironmentVariables: const {},
        ),
        const {
          'CARGO_ENCODED_RUSTFLAGS':
              '-C\x1fopt-level=2\x1f-C\x1f'
              'link-arg=-Wl,-z,max-page-size=16384\x1f-C\x1f'
              'link-arg=-Wl,-z,common-page-size=16384',
        },
      );
    });

    test('does not duplicate existing page-size flags', () {
      expect(
        cargoEnvironmentWithAndroidPageSize(
          targetOS: OS.android,
          targetArchitecture: Architecture.x64,
          parentEnvironment: const {},
          extraCargoEnvironmentVariables: const {
            'RUSTFLAGS': pageSizeRustFlags,
          },
        ),
        const {'RUSTFLAGS': pageSizeRustFlags},
      );
    });

    test('keeps non-Android targets unchanged', () {
      const extraCargoEnvironmentVariables = {'RUSTFLAGS': '-C opt-level=2'};

      expect(
        cargoEnvironmentWithAndroidPageSize(
          targetOS: OS.linux,
          targetArchitecture: Architecture.x64,
          parentEnvironment: const {},
          extraCargoEnvironmentVariables: extraCargoEnvironmentVariables,
        ),
        same(extraCargoEnvironmentVariables),
      );
    });

    test('keeps Android 32-bit targets unchanged', () {
      const extraCargoEnvironmentVariables = {'RUSTFLAGS': '-C opt-level=2'};

      expect(
        cargoEnvironmentWithAndroidPageSize(
          targetOS: OS.android,
          targetArchitecture: Architecture.arm,
          parentEnvironment: const {},
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
