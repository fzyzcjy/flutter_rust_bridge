import 'package:flutter_rust_bridge/src/droppable/_common.dart';
import 'package:test/test.dart';

void main() {
  test('DroppableDisposedException', () {
    expect(const DroppableDisposedException('sth').toString(),
        contains('DroppableDisposedException'));
  });
}
