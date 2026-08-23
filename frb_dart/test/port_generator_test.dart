import 'package:flutter_rust_bridge/src/utils/port_generator.dart';
import 'package:test/test.dart';

void main() {
  test('stream port names share a random context namespace', () {
    final first = ExecuteStreamPortGenerator.create('function');
    final second = ExecuteStreamPortGenerator.create('function');

    expect(
      first,
      matches(RegExp(r'^__frb_streamsink_[0-9a-f]{32}_function_0$')),
    );
    expect(
      second,
      matches(RegExp(r'^__frb_streamsink_[0-9a-f]{32}_function_1$')),
    );
    expect(
      first.replaceFirst(RegExp(r'_0$'), ''),
      second.replaceFirst(RegExp(r'_1$'), ''),
    );
  });
}
