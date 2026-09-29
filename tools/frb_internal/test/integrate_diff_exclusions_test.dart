import 'package:flutter_rust_bridge_internal/src/makefile_dart/integrate_diff_exclusions.dart';
import 'package:test/test.dart';

void main() {
  test('integrate extra args are explicit for flutter_via_create', () {
    const package = 'frb_example/flutter_via_create';
    expect(
      integrateDiffExclusionArgsForTesting(package, needCompareOhos: false),
      "':(exclude)$package/macos/Flutter/Flutter-Debug.xcconfig' "
      "':(exclude)$package/macos/Flutter/Flutter-Release.xcconfig' "
      "':(exclude)$package/pubspec.lock' "
      "':(exclude)$package/pubspec.yaml' "
      "':(exclude)$package/lib/src/rust/api/ohos_smoke.dart' "
      "':(exclude)$package/lib/src/rust/frb_generated.dart' "
      "':(exclude)$package/lib/src/rust/frb_generated.io.dart' "
      "':(exclude)$package/lib/src/rust/frb_generated.web.dart' "
      "':(exclude)$package/rust/src/api/mod.rs' "
      "':(exclude)$package/rust/src/api/ohos_smoke.rs' "
      "':(exclude)$package/rust/src/frb_generated.rs' "
      "':(exclude)$package/ohos/entry/src/main/module.json5' "
      "':(exclude)$package/ohos/ohos_device_smoke_main.dart' "
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
        "':(exclude)$package/lib/src/rust/api/ohos_smoke.dart' "
        "':(exclude)$package/lib/src/rust/frb_generated.dart' "
        "':(exclude)$package/lib/src/rust/frb_generated.io.dart' "
        "':(exclude)$package/lib/src/rust/frb_generated.web.dart' "
        "':(exclude)$package/rust/src/api/mod.rs' "
        "':(exclude)$package/rust/src/api/ohos_smoke.rs' "
        "':(exclude)$package/rust/src/frb_generated.rs' "
        "':(exclude)$package/ohos/entry/src/main/module.json5' "
        "':(exclude)$package/ohos/ohos_device_smoke_main.dart' "
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
        "':(exclude)$package/example/macos/Flutter/Flutter-Release.xcconfig' "
        "':(exclude)$package/example/pubspec.lock'",
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
