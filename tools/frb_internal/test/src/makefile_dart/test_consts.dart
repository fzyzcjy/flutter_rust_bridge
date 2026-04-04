import 'package:flutter_rust_bridge_internal/src/makefile_dart/consts.dart';
import 'package:test/test.dart';

void main() {
  test('kPrecommitGeneratePackages excludes deliberate_bad examples', () {
    expect(kPrecommitGeneratePackages, isNot(contains('frb_example/deliberate_bad')));
    expect(kPrecommitGeneratePackages, contains('frb_example/dart_minimal'));
    expect(kPrecommitGeneratePackages, contains('frb_example/flutter_package'));
  });
}
