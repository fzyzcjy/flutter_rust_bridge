import 'package:flutter_rust_bridge/src/droppable/_common.dart';
import 'package:test/test.dart';

void main() {
  test('DroppableDisposedException identifies the disposed runtime type', () {
    expect(
      const DroppableDisposedException('sth').toString(),
      'DroppableDisposedException: Try to use `sth` after it has been disposed',
    );
  });
}
