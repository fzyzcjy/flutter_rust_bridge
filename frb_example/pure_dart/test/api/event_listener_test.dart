import 'package:frb_example_pure_dart/src/rust/api/event_listener.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('dart register event listener & create event with delay', () async {
    expectLater(
        registerEventListener(), emits(Event(address: 'foo', payload: 'bar')));
    await Future.delayed(const Duration(milliseconds: 20));
    await createEvent(address: 'foo', payload: 'bar');
    await closeEventListener();
  });
}
