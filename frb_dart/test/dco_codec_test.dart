@TestOn('browser')
library;

import 'dart:js_interop';
import 'dart:typed_data';

import 'package:flutter_rust_bridge/src/codec/dco.dart';
import 'package:flutter_rust_bridge/src/manual_impl/_web.dart';
import 'package:test/test.dart';

enum _Color { red, green, blue }

String _decodeString(dynamic raw) => raw as String;

// Mirrors the cast pattern that FRB codegen emits for `int`-typed values.
// `dartify()` on dart2wasm hands us `double` for JS numbers, so the generated
// code routes through `num` before reaching `int`.
int _decodeInt(dynamic raw) => (raw as num).toInt();

List<int> _decodeListInt(dynamic raw) =>
    (raw as List<dynamic>).map(_decodeInt).toList();

({int a, String b}) _decodeRecord(dynamic raw) {
  final arr = raw as List<dynamic>;
  return (a: _decodeInt(arr[0]), b: arr[1] as String);
}

double _decodeDouble(dynamic raw) => raw as double;

bool _decodeBool(dynamic raw) => raw as bool;

// Mirrors the FRB codegen template for `char` (post-fix). `String.fromCharCode`
// is strictly typed as `int`, and dart2wasm `dartify()` hands us `double`.
String _decodeChar(dynamic raw) => String.fromCharCode((raw as num).toInt());

// Mirrors the FRB codegen template for primitive enums (post-fix).
_Color _decodeColor(dynamic raw) => _Color.values[(raw as num).toInt()];

// Mirrors `dco_decode_opt_*` from FRB codegen: null-check, then recursive
// decode of the inner type.
int? _decodeOptInt(dynamic raw) => raw == null ? null : _decodeInt(raw);

Uint8List _decodeUint8List(dynamic raw) => raw as Uint8List;

// Mirrors the FRB codegen template for *complex* enums (variants with data).
// `enumeration.rs` emits `switch (raw[0]) { case 0: ...; case 1: ...; }`.
// On dart2wasm `raw[0]` is `double`, and Dart's `==` makes `0 == 0.0` true,
// so the `case 0:` arm matches — but pin it with a test to guarantee.
sealed class _Shape {}

class _Circle extends _Shape {
  final int radius;
  _Circle(this.radius);
}

class _Square extends _Shape {
  final String label;
  _Square(this.label);
}

_Shape _decodeShape(dynamic raw) {
  switch (raw[0]) {
    case 0:
      return _Circle(_decodeInt(raw[1]));
    case 1:
      return _Square(raw[1] as String);
    default:
      throw Exception('unreachable');
  }
}

// Mirrors `dco_decode_i_64` from FRB codegen: delegates to the runtime
// helper which already handles the JSBigInt path defensively.
BigInt _decodeI64(dynamic raw) => dcoDecodeI64(raw);

@JS('BigInt')
external JSBigInt _jsBigIntCtor(JSAny value);

void main() {
  group('DcoCodec.decodeObject', () {
    test('decodes a top-level JS Array as a success message', () {
      // The wire shape from Rust through wasm-bindgen is a JS Array
      // `[action_code, payload]`. Under dart2wasm a JS Array crosses the
      // interop boundary as `JSArray<JSAny?>` rather than `List<dynamic>`,
      // so the original `raw as List<dynamic>` cast in `decodeObject` fails
      // without the `maybeDartify` boundary conversion this test guards.
      final raw = <JSAny?>[0.toJS, 'hello'.toJS].toJS;

      const codec = DcoCodec<String, Object>(
        decodeSuccessData: _decodeString,
        decodeErrorData: null,
      );

      expect(codec.decodeObject(raw), 'hello');
    });

    test('decodes a nested JS Array (dartify recurses into the payload)', () {
      // Inner array stands in for a `Vec<i32>` payload — exercises that
      // `dartify()` walks recursively so the inner `raw as List<dynamic>`
      // cast in generated decoders succeeds without further conversion.
      final raw = <JSAny?>[
        0.toJS,
        <JSAny?>[1.toJS, 2.toJS, 3.toJS].toJS,
      ].toJS;

      const codec = DcoCodec<List<int>, Object>(
        decodeSuccessData: _decodeListInt,
        decodeErrorData: null,
      );

      expect(codec.decodeObject(raw), [1, 2, 3]);
    });

    test('decodes a struct-shaped payload (mixed element types)', () {
      final raw = <JSAny?>[
        0.toJS,
        <JSAny?>[42.toJS, 'world'.toJS].toJS,
      ].toJS;

      const codec = DcoCodec<({int a, String b}), Object>(
        decodeSuccessData: _decodeRecord,
        decodeErrorData: null,
      );

      expect(codec.decodeObject(raw), (a: 42, b: 'world'));
    });

    test('decodes a double payload without coercion', () {
      // Guards against accidentally narrowing `double` to `int` — the fix only
      // routes `int`-typed casts through `(raw as num).toInt()`; `double` casts
      // remain direct.
      final raw = <JSAny?>[0.toJS, 1.5.toJS].toJS;

      const codec = DcoCodec<double, Object>(
        decodeSuccessData: _decodeDouble,
        decodeErrorData: null,
      );

      expect(codec.decodeObject(raw), 1.5);
    });

    test('decodes a bool payload', () {
      final raw = <JSAny?>[0.toJS, true.toJS].toJS;

      const codec = DcoCodec<bool, Object>(
        decodeSuccessData: _decodeBool,
        decodeErrorData: null,
      );

      expect(codec.decodeObject(raw), isTrue);
    });

    test('decodes a char payload via fromCharCode', () {
      final raw = <JSAny?>[0.toJS, 65.toJS].toJS;

      const codec = DcoCodec<String, Object>(
        decodeSuccessData: _decodeChar,
        decodeErrorData: null,
      );

      expect(codec.decodeObject(raw), 'A');
    });

    test('decodes a primitive enum via index lookup', () {
      // Exercises the `enum.values[(raw as num).toInt()]` pattern emitted by
      // `delegate.rs` for primitive enums under dart2wasm.
      final raw = <JSAny?>[0.toJS, 2.toJS].toJS;

      const codec = DcoCodec<_Color, Object>(
        decodeSuccessData: _decodeColor,
        decodeErrorData: null,
      );

      expect(codec.decodeObject(raw), _Color.blue);
    });

    test('decodes a null optional payload', () {
      // `dco_decode_opt_*` short-circuits on null before any cast can run, so
      // the null payload propagates through `decodeObject` unchanged.
      final raw = <JSAny?>[0.toJS, null].toJS;

      const codec = DcoCodec<int?, Object>(
        decodeSuccessData: _decodeOptInt,
        decodeErrorData: null,
      );

      expect(codec.decodeObject(raw), isNull);
    });

    test('decodes a present optional payload', () {
      final raw = <JSAny?>[0.toJS, 7.toJS].toJS;

      const codec = DcoCodec<int?, Object>(
        decodeSuccessData: _decodeOptInt,
        decodeErrorData: null,
      );

      expect(codec.decodeObject(raw), 7);
    });

    test('decodes a negative int payload', () {
      // `.toInt()` is direction-agnostic; verifies negative values survive the
      // num→int coercion on dart2wasm.
      final raw = <JSAny?>[0.toJS, (-42).toJS].toJS;

      const codec = DcoCodec<int, Object>(
        decodeSuccessData: _decodeInt,
        decodeErrorData: null,
      );

      expect(codec.decodeObject(raw), -42);
    });

    test('decodes a complex enum (switch on discriminant)', () {
      // Variant 0 = _Circle(5)
      final circle = <JSAny?>[
        0.toJS,
        <JSAny?>[0.toJS, 5.toJS].toJS,
      ].toJS;
      const circleCodec = DcoCodec<_Shape, Object>(
        decodeSuccessData: _decodeShape,
        decodeErrorData: null,
      );
      final circleResult = circleCodec.decodeObject(circle);
      expect(circleResult, isA<_Circle>());
      expect((circleResult as _Circle).radius, 5);

      // Variant 1 = _Square("box")
      final square = <JSAny?>[
        0.toJS,
        <JSAny?>[1.toJS, 'box'.toJS].toJS,
      ].toJS;
      const squareCodec = DcoCodec<_Shape, Object>(
        decodeSuccessData: _decodeShape,
        decodeErrorData: null,
      );
      final squareResult = squareCodec.decodeObject(square);
      expect(squareResult, isA<_Square>());
      expect((squareResult as _Square).label, 'box');
    });

    test('decodes an i64 payload as Dart BigInt', () {
      // Rust i64 crosses the boundary as a JS BigInt — verify
      // `dcoDecodeI64` survives the `dartify()` transformation.
      final raw = <JSAny?>[
        0.toJS,
        _jsBigIntCtor('9223372036854775000'.toJS),
      ].toJS;

      const codec = DcoCodec<BigInt, Object>(
        decodeSuccessData: _decodeI64,
        decodeErrorData: null,
      );

      expect(codec.decodeObject(raw), BigInt.parse('9223372036854775000'));
    });

    test('decodes a Uint8List payload (typed array)', () {
      // `dartify()` preserves JS Uint8Array as Dart `Uint8List` on both
      // compilers (the SSE codec already relies on this at sse.dart:31).
      final bytes = Uint8List.fromList([1, 2, 3, 255]);
      final raw = <JSAny?>[0.toJS, bytes.toJS].toJS;

      const codec = DcoCodec<Uint8List, Object>(
        decodeSuccessData: _decodeUint8List,
        decodeErrorData: null,
      );

      expect(codec.decodeObject(raw), bytes);
    });
  });
}
