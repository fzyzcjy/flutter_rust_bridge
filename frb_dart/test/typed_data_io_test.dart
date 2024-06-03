@TestOn('vm')
import 'dart:typed_data' as $data;
import 'dart:typed_data';

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:test/test.dart';

void main() {
  test('Int64List more operations', () {
    expect(Int64List.view(Uint8List(0).buffer).length, 0);
    expect(Int64List.sublistView(Uint8List(0)).length, 0);
    expect(Int64List.fromList([10]) + Int64List.fromList([20]),
        Int64List.fromList([10, 20]));
    expect(Int64List.fromList([10]) + $data.Int64List.fromList([20]),
        Int64List.fromList([10, 20]));
    expect(Int64List.fromList([10]) + [20], Int64List.fromList([10, 20]));
    expect(Int64List.fromList([10]) + [20].map((x) => x),
        Int64List.fromList([10, 20]));
    expect(() => Int64List.fromList([10]) + 42, throwsA(isA<ArgumentError>()));
    expect(() => Int64List.fromList([10]).length = 100,
        throwsA(isA<UnmodifiableTypedListException>()));
    // expect(() => Int64List.fromList([10]).dart2raw(0.5),
    //     throwsA(isA<ArgumentError>()));
  });

  test('Uint64List more operations', () {
    expect(Uint64List.view(Uint8List(0).buffer).length, 0);
    expect(Uint64List.sublistView(Uint8List(0)).length, 0);
    expect(Uint64List.fromList([10]) + Uint64List.fromList([20]),
        Uint64List.fromList([10, 20]));
    expect(Uint64List.fromList([10]) + $data.Uint64List.fromList([20]),
        Uint64List.fromList([10, 20]));
    expect(Uint64List.fromList([10]) + [20], Uint64List.fromList([10, 20]));
    expect(Uint64List.fromList([10]) + [20].map((x) => x),
        Uint64List.fromList([10, 20]));
    expect(() => Uint64List.fromList([10]) + 42, throwsA(isA<ArgumentError>()));
    expect(() => Uint64List.fromList([10]).length = 100,
        throwsA(isA<UnmodifiableTypedListException>()));
    // expect(() => Uint64List.fromList([10]).dart2raw(0.5),
    //     throwsA(isA<ArgumentError>()));
  });
}
