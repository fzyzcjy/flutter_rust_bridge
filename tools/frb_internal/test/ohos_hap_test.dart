import 'package:flutter_rust_bridge_internal/src/makefile_dart/ohos_hap.dart';
import 'package:test/test.dart';

void main() {
  test('OHOS HAP validation requires the arm64 Rust dynamic library', () {
    expect(
      ohosHapContainsRustLibraryForTesting([
        'libs/arm64-v8a/libflutter.so',
        'libs/arm64-v8a/librust_lib_example.so',
      ], expectedLibrary: 'librust_lib_example.so'),
      isTrue,
    );
    expect(
      ohosHapContainsRustLibraryForTesting([
        'libs/x86_64/librust_lib_example.so',
      ], expectedLibrary: 'librust_lib_example.so'),
      isFalse,
    );
    expect(
      ohosHapContainsRustLibraryForTesting([
        'assets/arm64-v8a/librust_lib_example.so',
      ], expectedLibrary: 'librust_lib_example.so'),
      isFalse,
    );
    expect(
      ohosHapContainsRustLibraryForTesting([
        'libs/arm64-v8a/libflutter.so',
      ], expectedLibrary: 'librust_lib_example.so'),
      isFalse,
    );
  });

  test('OHOS HAP inspection falls back from JDK jar to unzip', () {
    final candidates = ohosArchiveListerCandidatesForTesting();
    expect(candidates.map((candidate) => candidate.executable), [
      'jar',
      'unzip',
    ]);
    expect(candidates[0].arguments, ['--version']);
    expect(candidates[1].arguments, ['-v']);
  });

  test('OHOS device smoke validates the HAP bundle name', () {
    expect(
      ohosHapBundleNameForTesting('''
{"summary":{"app":{"bundleName":"com.example.smoke"}}}
'''),
      'com.example.smoke',
    );
    expect(
      () => ohosHapBundleNameForTesting('{"summary":{"app":{}}}'),
      throwsFormatException,
    );
    expect(() => ohosHapBundleNameForTesting('[]'), throwsFormatException);
  });
}
