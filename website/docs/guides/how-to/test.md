# Testing and mocking

In this section, we discuss some of the ways to test an application / library
that uses flutter_rust_bridge.

## Testing Dart code without Rust code

A common practice in testing is to check the layers one by one.
Suppose we want to test the Dart code without the real Rust code.
Then, we can do mocking on the `RustLibApi` abstract class (or other name if you customized the names).
For example, we can use [mockito](https://pub.dev/packages/mockito),
[mocktail](https://pub.dev/packages/mocktail), etc.

`RustLibApi` class is designed with testability and mockability in mind.
It is designed such that all generated functions must eventually call methods in this class.
Or, think of it like a "central dispatcher".
Therefore, as long as you mocked this class, everything related to the Rust side is under your mock.

Please refer to [this file](https://github.com/fzyzcjy/flutter_rust_bridge/blob/master/frb_example/pure_dart/test/mockability_test.dart)
for an example (and it is tested on the CI).
The code may look like:

```dart
// Surely, you can use Mockito or whatever other mocking packages
class MockRustLibApi extends Mock implements RustLibApi {}

Future<void> main() async {
  final mockApi = MockRustLibApi();
  await RustLib.init(api: mockApi);

  test('can mock Rust calls', () async {
    when(() => mockApi.simpleAdderTwinNormal(a: 1, b: 2))
        .thenAnswer((_) async => 123456789);
    final actualResult = await simpleAdderTwinNormal(a: 1, b: 2);
    expect(actualResult, isNot(3));
    expect(actualResult, equals(123456789));
    verify(() => mockApi.simpleAdderTwinNormal(a: 1, b: 2)).called(1);
  });
}
```

## Testing Rust code without Dart code

Indeed just use standard methods to test the standard Rust code -
there is nothing special about the Rust code in your app.
For example, [the Rust book](https://doc.rust-lang.org/book/ch11-00-testing.html) explains how to do it.

## Testing Dart code and Rust code

Similarly, just use standard Flutter/Dart testing techniques.
If you want examples, have a look at various packages in `frb_example`.
Our CI runs the tests on every commit.

By default, the Rust compilation and Rust library loading should be done
automatically without manual intervention.
In other words, there is no need to manually configure anything in order to make tests run.
