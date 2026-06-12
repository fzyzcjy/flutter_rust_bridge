import 'package:flutter_rust_bridge_internal/src/makefile_dart/integrate_diff_exclusions.dart';
import 'package:test/test.dart';

void main() {
  test('generate extra args are explicit for flutter_via_create', () {
    for (final package in [
      'frb_example/flutter_via_create',
      'frb_example/flutter_via_create_native_assets',
    ]) {
      expect(
        generateDiffExclusionArgsForTesting(package),
        "':(exclude)$package/macos/Flutter/Flutter-Debug.xcconfig' "
        "':(exclude)$package/macos/Flutter/Flutter-Release.xcconfig'",
        reason: package,
      );
    }
  });

  test('generate extra args are explicit for flutter_via_integrate', () {
    for (final package in [
      'frb_example/flutter_via_integrate',
      'frb_example/flutter_via_integrate_native_assets',
    ]) {
      expect(
        generateDiffExclusionArgsForTesting(package),
        "':(exclude)$package/macos/Flutter/Flutter-Debug.xcconfig' "
        "':(exclude)$package/macos/Flutter/Flutter-Release.xcconfig'",
        reason: package,
      );
    }
  });

  test('generate extra args are explicit for flutter_package', () {
    for (final package in [
      'frb_example/flutter_package',
      'frb_example/flutter_package_native_assets',
    ]) {
      expect(
        generateDiffExclusionArgsForTesting(package),
        "':(exclude)$package/example/macos/Flutter/Flutter-Debug.xcconfig' "
        "':(exclude)$package/example/macos/Flutter/Flutter-Release.xcconfig'",
        reason: package,
      );
    }
  });

  test('generate extra args are empty for unrelated package', () {
    expect(generateDiffExclusionArgsForTesting('frb_example/gallery'), isEmpty);
  });

  test('integrate extra args are explicit for flutter_via_create', () {
    const package = 'frb_example/flutter_via_create';
    expect(
      integrateDiffExclusionArgsForTesting(package, needCompareOhos: false),
      "':(exclude)$package/macos/Flutter/Flutter-Debug.xcconfig' "
      "':(exclude)$package/macos/Flutter/Flutter-Release.xcconfig' "
      "':(exclude)$package/pubspec.lock' "
      "':(exclude)$package/pubspec.yaml' "
      "':(exclude)$package/ohos/' "
      "':(exclude)$package/rust_builder/ohos/' "
      "':(exclude)$package/rust_builder/pubspec.yaml'",
    );
  });

  test(
    'integrate extra args are explicit for flutter_via_create_native_assets',
    () {
      const package = 'frb_example/flutter_via_create_native_assets';
      expect(
        integrateDiffExclusionArgsForTesting(package, needCompareOhos: false),
        "':(exclude)$package/macos/Flutter/Flutter-Debug.xcconfig' "
        "':(exclude)$package/macos/Flutter/Flutter-Release.xcconfig' "
        "':(exclude)$package/pubspec.lock' "
        "':(exclude)$package/pubspec.yaml' "
        "':(exclude)$package/ohos/'",
      );
    },
  );

  test(
    'integrate extra args compare ohos for flutter_via_create when requested',
    () {
      const package = 'frb_example/flutter_via_create';
      expect(
        integrateDiffExclusionArgsForTesting(package, needCompareOhos: true),
        "':(exclude)$package/macos/Flutter/Flutter-Debug.xcconfig' "
        "':(exclude)$package/macos/Flutter/Flutter-Release.xcconfig' "
        "':(exclude)$package/pubspec.lock' "
        "':(exclude)$package/pubspec.yaml' "
        "':(exclude)$package/android/' "
        "':(exclude)$package/macos/' "
        "':(exclude)$package/windows/'",
      );
    },
  );

  test('integrate extra args are explicit for flutter_via_integrate', () {
    for (final package in [
      'frb_example/flutter_via_integrate',
      'frb_example/flutter_via_integrate_native_assets',
    ]) {
      expect(
        integrateDiffExclusionArgsForTesting(package, needCompareOhos: false),
        "':(exclude)$package/macos/Flutter/Flutter-Debug.xcconfig' "
        "':(exclude)$package/macos/Flutter/Flutter-Release.xcconfig'",
        reason: package,
      );
    }
  });

  test('integrate extra args are explicit for flutter_package', () {
    for (final package in [
      'frb_example/flutter_package',
      'frb_example/flutter_package_native_assets',
    ]) {
      expect(
        integrateDiffExclusionArgsForTesting(package, needCompareOhos: false),
        "':(exclude)$package/example/macos/Flutter/Flutter-Debug.xcconfig' "
        "':(exclude)$package/example/macos/Flutter/Flutter-Release.xcconfig'",
        reason: package,
      );
    }
  });

  test('integrate extra args are empty for unrelated package', () {
    expect(
      integrateDiffExclusionArgsForTesting(
        'frb_example/gallery',
        needCompareOhos: false,
      ),
      isEmpty,
    );
  });
}
