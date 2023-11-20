import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/api/simple.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main() async {
  final customHandler = _MyHandler();
  await FrbExamplePureDart.init(dispatcher: FrbExamplePureDartDispatcher(handler: customHandler));

  test('can use custom subclasses', () async {
    expect(await simpleAdder(a: 1, b: 2), 3);
    expect(logsFromCustomSubclass, TODO);
  });
}

class _MyHandler extends BaseHandler {}
