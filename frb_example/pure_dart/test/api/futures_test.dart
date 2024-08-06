import 'package:frb_example_pure_dart/src/rust/api/futures.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  group('functions returning futures', () {
    test('dart call `impl Future` adder', () async {
      expect(await implFutureAdder(a: 100, b: 200), 300);
    });

    test('dart call `DartFnFuture` adder', () async {
      expect(await dartfnFutureAdder(a: 100, b: 200, c: 300), 600);
    });

    test('dart call `BoxFuture` adder', () async {
      expect(await boxFutureAdder(a: 100, b: 200, c: 300, d: 400), 1000);
    });

    test('dart call `impl Future` adder returning `Result::Ok`', () async {
      expect(await implFutureAdderResult(a: 100, b: 200, true), 300);
    });

    test('dart call `impl Future` adder returning `Result::Err`', () async {
      expect(await expectLater(
          () async => implFutureAdderResult(a: 100, b: 200, false),
          throwsA(isA<CustomErr>())));
    });

    test('dart call `DartFnFuture` adder returning `Result::Ok`', () async {
      expect(await dartfnFutureAdderResult(a: 100, b: 200, c: 300, true), 600);
    });

    test('dart call `DartFnFuture` adder returning `Result::Err`', () async {
      expect(await expectLater(
          () async => dartfnFutureAdderResult(a: 100, b: 200, c: 300, false),
          throwsA(isA<CustomErr>())));

    });

    test('dart call `BoxFuture` adder returning `Result::Ok`', () async {
      expect(await boxFutureAdder(a: 100, b: 200, c: 300, d: 400, true), 1000);
    });

    test('dart call `BoxFuture` adder returning `Result::Err`', () async {
      expect(await expectLater(
          () async => boxFutureAdder(a: 100, b: 200, c: 300, d: 400, false),
          throwsA(isA<CustomErr>())));
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




// #[frb(opaque)]
// pub struct StructWithAsyncMethods {
//     name: String
// }

// impl StructWithAsyncMethods {
//     #[frb(sync)]
//     pub fn new(name: &str) -> Self {
//         Self {
//             name: name.to_string()
//         }
//     }

//     pub fn impl_future_hello(&self) -> impl Future<Output = Result<String, MyErr>> {
//        async {
//             format("Hello, {}", &self.name)
//         }
//     }

//     pub fn dartfn_future_hello(&self) -> DartFnFuture<String> {
//         Box::pin(async {
//             format("Bonjour, {}", &self.name)
//         })
//     }

//     pub fn box_future_hello(&self) -> Pin<Box<dyn Future<Output = String> + Send + 'static>> {
//         Box::pin(async {
//             format("Hola, {}", &self.name)
//         })
//     }

//     pub fn impl_future_hello_result(&self, succeed: bool) -> impl Future<Output = Result<String, MyErr>> {
//        async {
//             match succeed {
//                 true => Ok(format("Hello, {}", &self.name)),
//                 false => Err(CustomErr::Failure)
//             }
//         }
//     }

//     pub fn dartfn_future_hello_result(&self, succeed: bool) -> DartFnFuture<String> {
//         Box::pin(async {
//             match succeed {
//                 true => Ok(format("Bonjour, {}", &self.name)),
//                 false => Err(CustomErr::Failure)
//             }
//         })
//     }

//     pub fn box_future_hello_result(&self, succeed: bool) -> Pin<Box<dyn Future<Output = String> + Send + 'static>> {
//         Box::pin(async {
//             match succeed {
//                 true => Ok(format("Hola, {}", &self.name)),
//                 false => Err(CustomErr::Failure)
//             }
//         })
//     }

// }

// pub trait TraitWithAsyncMethods {
//     fn example_async_method(
//         &mut self,
//         arg_one: u8,
//         arg_two: u32,
//         arg_three: Vec<u8>,
//     ) -> impl Future<Output = Result<Vec<u8>, CustomErr>>;

// }