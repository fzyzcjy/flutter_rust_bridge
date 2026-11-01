import 'package:flutter_rust_bridge_internal/src/makefile_dart/generate.dart';
import 'package:test/test.dart';

void main() {
  test('integrate OHOS generation asks Flutter for the OHOS scaffold', () {
    expect(
      flutterCreateCommandForIntegrateForTesting(
        packageName: 'flutter_via_integrate',
        includeOhos: true,
      ),
      'flutter create flutter_via_integrate '
      '--platforms android,ios,linux,macos,ohos,web,windows',
    );
  });

  test('ordinary integrate generation keeps Flutter defaults', () {
    expect(
      flutterCreateCommandForIntegrateForTesting(
        packageName: 'flutter_via_integrate',
        includeOhos: false,
      ),
      'flutter create flutter_via_integrate',
    );
  });
}
