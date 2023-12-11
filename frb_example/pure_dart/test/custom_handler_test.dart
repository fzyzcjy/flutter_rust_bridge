import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/simple.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  final customHandler = _MyHandler();
  await RustLib.init(handler: customHandler);

  test('can use custom handler', () async {
    expect(customHandler.logs, <String>[]);
    expect(await simpleAdderTwinNormal(a: 1, b: 2), 3);
    expect(customHandler.logs, ['executeNormal called']);
  });
}

class _MyHandler extends BaseHandler {
  final logs = <String>[];

  @override
  Future<S> executeNormal<S, E extends Object>(NormalTask<S, E> task) {
    logs.add('executeNormal called');
    return super.executeNormal(task);
  }
}
