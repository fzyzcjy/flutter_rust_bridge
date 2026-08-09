import 'package:flutter_rust_bridge/src/utils/port_generator.dart';
import 'package:test/test.dart';

/// Verifies stream port names contain a stable session nonce and unique index.
void main() {
  test('stream port names are session-scoped and unique', () {
    final first = ExecuteStreamPortGenerator.create('test');
    final second = ExecuteStreamPortGenerator.create('test');
    final pattern = RegExp(r'^__frb_streamsink_([0-9a-f]{32})_test_[0-9]+$');
    final firstMatch = pattern.firstMatch(first);
    final secondMatch = pattern.firstMatch(second);

    expect(firstMatch, isNotNull);
    expect(secondMatch, isNotNull);
    expect(firstMatch!.group(1), secondMatch!.group(1));
    expect(first, isNot(second));
  });
}
