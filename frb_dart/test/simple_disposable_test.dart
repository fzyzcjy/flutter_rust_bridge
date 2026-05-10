@TestOn('vm')
import 'package:flutter_rust_bridge/src/misc/simple_disposable.dart';
import 'package:test/test.dart';

void main() {
  test('SimpleDisposable reports disposed state after dispose is called', () {
    final disposable = _Disposable();

    expect(disposable.isDisposed, isFalse);

    disposable.dispose();

    expect(disposable.isDisposed, isTrue);
  });
}

class _Disposable with SimpleDisposable {}
