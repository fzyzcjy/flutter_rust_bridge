import 'dart:ffi';
import 'dart:io';

import 'package:build_tool/src/android_environment.dart';
import 'package:build_tool/src/target.dart';
import 'package:path/path.dart' as path;
import 'package:test/test.dart';

void main() {
  late Directory tempDir;

  setUp(() {
    tempDir = Directory.systemTemp.createTempSync('android_environment_');
  });

  tearDown(() {
    tempDir.deleteSync(recursive: true);
  });

  test('prefers darwin arm64 NDK toolchain on Apple Silicon', () async {
    final env = _createEnvironment(
      tempRoot: tempDir.path,
      hostDir: 'darwin-arm64',
      hostAbi: Abi.macosArm64,
      toolLayout: _ToolLayout.legacy,
    );

    final buildEnvironment = await env.buildEnvironment();

    expect(
      buildEnvironment['CC_aarch64-linux-android'],
      contains('darwin-arm64'),
    );
    expect(
      buildEnvironment['AR_aarch64-linux-android'],
      contains('darwin-arm64'),
    );
  });

  test('falls back to darwin x64 NDK toolchain when needed', () async {
    final env = _createEnvironment(
      tempRoot: tempDir.path,
      hostDir: 'darwin-x86_64',
      hostAbi: Abi.macosArm64,
      toolLayout: _ToolLayout.legacy,
    );

    final buildEnvironment = await env.buildEnvironment();

    expect(
      buildEnvironment['CC_aarch64-linux-android'],
      contains('darwin-x86_64'),
    );
  });

  test('uses llvm binutils from newer NDKs when prefixed tools are absent',
      () async {
    final env = _createEnvironment(
      tempRoot: tempDir.path,
      hostDir: 'darwin-arm64',
      hostAbi: Abi.macosArm64,
      toolLayout: _ToolLayout.llvmOnly,
    );

    final buildEnvironment = await env.buildEnvironment();

    expect(
      path.basename(buildEnvironment['AR_aarch64-linux-android']!),
      'llvm-ar',
    );
    expect(
      path.basename(buildEnvironment['RANLIB_aarch64-linux-android']!),
      'llvm-ranlib',
    );
  });
}

AndroidEnvironment _createEnvironment({
  required String tempRoot,
  required String hostDir,
  required Abi hostAbi,
  required _ToolLayout toolLayout,
}) {
  _createPrebuilt(tempRoot, hostDir, toolLayout);
  return AndroidEnvironment(
    sdkPath: path.join(tempRoot, 'sdk'),
    ndkVersion: '26.1.10909125',
    minSdkVersion: 21,
    targetTempDir: path.join(tempRoot, 'build-temp'),
    target: Target.forRustTriple('aarch64-linux-android')!,
    hostAbi: hostAbi,
    buildToolRunnerPath: path.join(tempRoot, 'run_build_tool.sh'),
  );
}

String _createPrebuilt(
  String tempRoot,
  String hostDir,
  _ToolLayout toolLayout,
) {
  final ndkRoot = path.join(
    tempRoot,
    'sdk',
    'ndk',
    '26.1.10909125',
    'toolchains',
    'llvm',
    'prebuilt',
    hostDir,
    'bin',
  );
  Directory(ndkRoot).createSync(recursive: true);
  switch (toolLayout) {
    case _ToolLayout.legacy:
      File(path.join(ndkRoot, 'aarch64-linux-android-ar')).writeAsStringSync(
        '',
      );
      File(path.join(ndkRoot, 'aarch64-linux-android-ranlib'))
          .writeAsStringSync('');
    case _ToolLayout.llvmOnly:
      File(path.join(ndkRoot, 'llvm-ar')).writeAsStringSync('');
      File(path.join(ndkRoot, 'llvm-ranlib')).writeAsStringSync('');
  }
  File(path.join(ndkRoot, 'clang')).writeAsStringSync('');
  File(path.join(ndkRoot, 'clang++')).writeAsStringSync('');
  return ndkRoot;
}

enum _ToolLayout {
  legacy,
  llvmOnly,
}
