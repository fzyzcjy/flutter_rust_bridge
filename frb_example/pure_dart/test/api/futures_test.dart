import 'package:frb_example_pure_dart/src/rust/api/futures.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('functions returning futures', () {
    test('dart call `impl Future` adder', () async {
      expect(await implFutureAdderTwinNormal(a: 100, b: 200), 300);
    });

    test('dart call `DartFnFuture` adder', () async {
      expect(await dartfnFutureAdderTwinNormal(a: 100, b: 200, c: 300), 600);
    });

    test('dart call `BoxFuture` adder', () async {
      expect(await boxFutureAdder(a: 100, b: 200, c: 300, d: 400), 1000);
    });

    test('dart call `impl Future` adder returning `Result::Ok`', () async {
      expect(await implFutureAdderResultTwinNormal(a: 100, b: 200, true), 300);
    });

    test('dart call `impl Future` adder returning `Result::Err`', () async {
      expect(await expectLater(
          () async => implFutureAdderResultTwinNormal(a: 100, b: 200, false),
          throwsA(isA<CustomErrTwinNormal>())));
    });

    test('dart call `DartFnFuture` adder returning `Result::Ok`', () async {
      expect(
          await dartfnFutureAdderResultTwinNormal(a: 100, b: 200, c: 300, true),
          600);
    });

    test('dart call `DartFnFuture` adder returning `Result::Err`', () async {
      expect(await expectLater(
          () async =>
              dartfnFutureAdderResultTwinNormal(a: 100, b: 200, c: 300, false),
          throwsA(isA<CustomErrTwinNormal>())));
    });

    test('dart call `BoxFuture` adder returning `Result::Ok`', () async {
      expect(
          await boxFutureAdderTwinNormal(a: 100, b: 200, c: 300, d: 400, true),
          1000);
    });

    test('dart call `BoxFuture` adder returning `Result::Err`', () async {
      expect(await expectLater(
          () async =>
              boxFutureAdderTwinNormal(a: 100, b: 200, c: 300, d: 400, false),
          throwsA(isA<CustomErrTwinNormal>())));
    });
  });

  group('methods returning futures', () {
    // test('example_async_future', () async {
    // var swam = StructWithAsyncMethods();
    //   print('Action: Call rust (before)');
    //   expect(await ms.exampleAsyncFuture(), "result_value_one");
    //   print('Action: Call rust (after)');
    // });

    // test('example_async_future2', () async {
    //   print('Action: Call rust (before)');
    //   expect(await ms.exampleAsyncFuture2(), "result_value_two");
    //   print('Action: Call rust (after)');
    // });

    // test('example_async_future_result', () async {
    //   print('Action: Call rust (before)');
    //   expect(await ms.exampleAsyncFutureResult(), "result_value_three");
    //   print('Action: Call rust (after)');
    // });

    // test('example_async_future_result2', () async {
    //   print('Action: Call rust (before)');
    //   expect(await ms.exampleAsyncFutureResult2(), "result_value_four");
    //   print('Action: Call rust (after)');
    // });
  });
}

// impl StructWithAsyncMethodsTwinNormal {
//     pub fn impl_future_hello(&self) -> impl Future<Output = Result<String, MyErr>> {
//             format("Hello, {}", &self.name)

//     pub fn dartfn_future_hello(&self) -> DartFnFuture<String> {
//             format("Bonjour, {}", &self.name)

//     pub fn box_future_hello(&self) -> Pin<Box<dyn Future<Output = String> + Send + 'static>> {
//             format("Hola, {}", &self.name)

// pub trait TraitWithAsyncMethodsTwinNormal {
//     fn example_async_method(
//         &mut self,
//         arg_one: u8,
//         arg_two: u32,
//         arg_three: Vec<u8>,
//     ) -> impl Future<Output = Result<Vec<u8>, CustomErr>>;

// }
