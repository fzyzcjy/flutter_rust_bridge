import 'package:flutter_rust_bridge/src/misc/box.dart';
import 'package:flutter_rust_bridge/src/misc/simple_disposable.dart';
import 'package:test/test.dart';

class _Disposable with SimpleDisposable {}

void main() {
  test('Box retains its wrapped value', () {
    const box = Box<String>('value');

    expect(box.value, 'value');
  });

  test('SimpleDisposable records disposal idempotently', () {
    final disposable = _Disposable();

    expect(disposable.isDisposed, isFalse);
    disposable.dispose();
    disposable.dispose();

    expect(disposable.isDisposed, isTrue);
  });
}
