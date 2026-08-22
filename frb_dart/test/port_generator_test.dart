import 'package:flutter_rust_bridge/src/utils/port_generator.dart';
import 'package:test/test.dart';

void main() {
  test('stream port generator distinguishes repeated function names', () {
    final first = ExecuteStreamPortGenerator.create('function');
    final second = ExecuteStreamPortGenerator.create('function');

    expect(first, '__frb_streamsink_function_0');
    expect(second, '__frb_streamsink_function_1');
  });

  test('lazy port generator allocates distinct monotonically named ports', () {
    final first = BaseLazyPortIdGenerator.create();
    final second = BaseLazyPortIdGenerator.create();

    expect(first, startsWith('__frb_lazy_port_'));
    expect(second, startsWith('__frb_lazy_port_'));
    expect(first, isNot(second));
  });
}
