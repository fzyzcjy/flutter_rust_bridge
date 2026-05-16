import 'package:flutter_rust_bridge_internal/src/makefile_dart/integrate_diff_exclusions.dart';
import 'package:test/test.dart';

void main() {
  test('integrate extra args are explicit for flutter_via_create', () {
    expect(
      integrateDiffExclusionArgsForTesting('frb_example/flutter_via_create'),
      "':(exclude)frb_example/flutter_via_create/macos/Flutter/Flutter-Debug.xcconfig' "
      "':(exclude)frb_example/flutter_via_create/macos/Flutter/Flutter-Release.xcconfig'",
    );
  });

  test('integrate extra args are explicit for flutter_via_integrate', () {
    expect(
      integrateDiffExclusionArgsForTesting('frb_example/flutter_via_integrate'),
      "':(exclude)frb_example/flutter_via_integrate/macos/Flutter/Flutter-Debug.xcconfig' "
      "':(exclude)frb_example/flutter_via_integrate/macos/Flutter/Flutter-Release.xcconfig'",
    );
  });

  test('integrate extra args are explicit for flutter_package', () {
    expect(
      integrateDiffExclusionArgsForTesting('frb_example/flutter_package'),
      "':(exclude)frb_example/flutter_package/example/macos/Flutter/Flutter-Debug.xcconfig' "
      "':(exclude)frb_example/flutter_package/example/macos/Flutter/Flutter-Release.xcconfig'",
    );
  });

  test('integrate extra args are empty for unrelated package', () {
    expect(
      integrateDiffExclusionArgsForTesting('frb_example/gallery'),
      isEmpty,
    );
  });
}
