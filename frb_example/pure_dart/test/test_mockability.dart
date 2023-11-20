import 'package:frb_example_pure_dart/src/rust/api/simple.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:mocktail/mocktail.dart';
import 'package:test/test.dart';

// Surely, you can use Mockito or whatever other mocking packages
class MockFrbGeneratedPureDartDispatcher extends Mock implements RustDispatcher {}

Future<void> main() async {
  final mockDispatcher = MockFrbGeneratedPureDartDispatcher();
  await Rust.init(dispatcher: mockDispatcher);

  test('can mock Rust calls', () async {
    when(() => mockDispatcher.simpleAdder(a: 1, b: 2)).thenAnswer((_) async => 123456789);
    final actualResult = await simpleAdder(a: 1, b: 2);
    expect(actualResult, isNot(3));
    expect(actualResult, equals(123456789));
    verify(() => mockDispatcher.simpleAdder(a: 1, b: 2)).called(1);
  });
}
