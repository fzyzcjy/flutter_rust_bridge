import 'dart:typed_data';

import 'package:frb_example_pure_dart/src/rust/api/result_oxidized.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';
import 'package:uuid/uuid.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('async oxidized Result returns Ok', () async {
    final result = await fallibleDivideOxidized(a: 10, b: 2);
    expect(result.isOk(), isTrue);
    expect(result.unwrap(), 5);
  });

  test('async oxidized Result returns Err', () async {
    final result = await fallibleDivideOxidized(a: 10, b: 0);
    expect(result.isErr(), isTrue);
    expect(result.unwrapErr().message, 'division by zero');
  });

  test('sync oxidized Result returns Ok', () {
    final result = fallibleDivideOxidizedSync(a: 10, b: 2);
    expect(result.isOk(), isTrue);
    expect(result.unwrap(), 5);
  });

  test('sync oxidized Result returns Err', () {
    final result = fallibleDivideOxidizedSync(a: 10, b: 0);
    expect(result.isErr(), isTrue);
    expect(result.unwrapErr().message, 'division by zero');
  });

  test('non-oxidized Result still throws on Err', () async {
    expect(
      () => fallibleDivideThrowing(a: 10, b: 0),
      throwsA(isA<ResultOxidizedError>()),
    );
  });

  test('oxidized Result still throws on panic', () async {
    expect(
      () => panicOxidizedResult(),
      throwsA(isA<PanicException>()),
    );
  });

  test('generic Result alias returns primitive Ok and Err', () {
    final ok = wresultAliasSync(a: 12, b: 3);
    expect(ok.isOk(), isTrue);
    expect(ok.unwrap(), 4);

    final err = wresultAliasSync(a: 12, b: 0);
    expect(err.isErr(), isTrue);
    expect(err.unwrapErr().message, 'division by zero');
  });

  test('generic Result alias returns deterministic Uuid', () async {
    final result = await wresultUuid();
    expect(result.isOk(), isTrue);
    expect(result.unwrap(), UuidValue.fromString('12345678-1234-5678-1234-567812345678'));
  });

  test('generic Result alias returns String Ok and Err', () {
    final ok = wresultStringSync(name: 'Alice');
    expect(ok.isOk(), isTrue);
    expect(ok.unwrap(), 'Hello, Alice!');

    final err = wresultStringSync(name: '');
    expect(err.isErr(), isTrue);
    expect(err.unwrapErr().message, 'name cannot be empty');
  });

  test('generic Result alias returns struct Ok and Err', () async {
    final ok = await wresultStruct(id: 7, name: 'Bob');
    expect(ok.isOk(), isTrue);
    expect(ok.unwrap(), ResultOxidizedUser(id: 7, name: 'Bob'));

    final err = await wresultStruct(id: -1, name: 'Bob');
    expect(err.isErr(), isTrue);
    expect(err.unwrapErr().message, 'id must be non-negative');
  });

  test('generic Result alias returns Vec Ok and Err', () async {
    final ok = await wresultVec(count: 4);
    expect(ok.isOk(), isTrue);
    expect(ok.unwrap(), isA<Int32List>());
    expect(ok.unwrap().toList(), [0, 1, 2, 3]);

    final err = await wresultVec(count: -1);
    expect(err.isErr(), isTrue);
    expect(err.unwrapErr().message, 'count must be non-negative');
  });

  test('generic Result alias returns Option Ok and Err', () async {
    final some = await wresultOption(value: 9);
    expect(some.isOk(), isTrue);
    expect(some.unwrap(), 'Value: 9');

    final none = await wresultOption();
    expect(none.isOk(), isTrue);
    expect(none.unwrap(), isNull);

    final err = await wresultOption(value: -1);
    expect(err.isErr(), isTrue);
    expect(err.unwrapErr().message, 'value must be non-negative');
  });

  test('multi-parameter generic alias returns tuple', () {
    final result = pairAliasSync(a: 42, b: 'answer');
    expect(result, (42, 'answer'));
  });

  test('nested generic Result alias returns Ok and Err', () async {
    final ok = await wresultNested(items: ['a', 'Bee']);
    expect(ok.isOk(), isTrue);
    expect(ok.unwrap(), ['A', 'BEE']);

    final err = await wresultNested(items: []);
    expect(err.isErr(), isTrue);
    expect(err.unwrapErr().message, 'items cannot be empty');
  });
}
