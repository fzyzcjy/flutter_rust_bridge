// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.35.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'stream_twin_rust_async.dart';

Future<
    Stream<
        String>> funcStreamReturnErrorTwinRustAsyncSse({dynamic hint}) => RustLib
    .instance.api
    .crateApiPseudoManualStreamTwinRustAsyncSseFuncStreamReturnErrorTwinRustAsyncSse(
        hint: hint);

Future<
    Stream<
        String>> funcStreamReturnPanicTwinRustAsyncSse({dynamic hint}) => RustLib
    .instance.api
    .crateApiPseudoManualStreamTwinRustAsyncSseFuncStreamReturnPanicTwinRustAsyncSse(
        hint: hint);

Stream<int> funcStreamSinkArgPositionTwinRustAsyncSse(
        {required int a, required int b, dynamic hint}) =>
    RustLib.instance.api
        .crateApiPseudoManualStreamTwinRustAsyncSseFuncStreamSinkArgPositionTwinRustAsyncSse(
            a: a, b: b, hint: hint);

Stream<MyStreamEntryTwinRustAsyncSse> handleStreamOfStructTwinRustAsyncSse(
        {dynamic hint}) =>
    RustLib.instance.api
        .crateApiPseudoManualStreamTwinRustAsyncSseHandleStreamOfStructTwinRustAsyncSse(
            hint: hint);

Stream<LogTwinRustAsyncSse> handleStreamSinkAt1TwinRustAsyncSse(
        {required int key, required int max, dynamic hint}) =>
    RustLib.instance.api
        .crateApiPseudoManualStreamTwinRustAsyncSseHandleStreamSinkAt1TwinRustAsyncSse(
            key: key, max: max, hint: hint);

Stream<LogTwinRustAsyncSse> handleStreamSinkAt2TwinRustAsyncSse(
        {required int key, required int max, dynamic hint}) =>
    RustLib.instance.api
        .crateApiPseudoManualStreamTwinRustAsyncSseHandleStreamSinkAt2TwinRustAsyncSse(
            key: key, max: max, hint: hint);

Stream<LogTwinRustAsyncSse> handleStreamSinkAt3TwinRustAsyncSse(
        {required int key, required int max, dynamic hint}) =>
    RustLib.instance.api
        .crateApiPseudoManualStreamTwinRustAsyncSseHandleStreamSinkAt3TwinRustAsyncSse(
            key: key, max: max, hint: hint);

Stream<U8Array2> streamSinkFixedSizedPrimitiveArrayTwinRustAsyncSse(
        {dynamic hint}) =>
    RustLib.instance.api
        .crateApiPseudoManualStreamTwinRustAsyncSseStreamSinkFixedSizedPrimitiveArrayTwinRustAsyncSse(
            hint: hint);

Future<void> streamSinkInsideVecTwinRustAsyncSse(
        {required List<RustStreamSink<int>> arg, dynamic hint}) =>
    RustLib.instance.api
        .crateApiPseudoManualStreamTwinRustAsyncSseStreamSinkInsideVecTwinRustAsyncSse(
            arg: arg, hint: hint);

Future<void> streamSinkInsideStructTwinRustAsyncSse(
        {required MyStructContainingStreamSinkTwinRustAsyncSse arg,
        dynamic hint}) =>
    RustLib.instance.api
        .crateApiPseudoManualStreamTwinRustAsyncSseStreamSinkInsideStructTwinRustAsyncSse(
            arg: arg, hint: hint);

class LogTwinRustAsyncSse {
  final int key;
  final int value;

  const LogTwinRustAsyncSse({
    required this.key,
    required this.value,
  });

  @override
  int get hashCode => key.hashCode ^ value.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is LogTwinRustAsyncSse &&
          runtimeType == other.runtimeType &&
          key == other.key &&
          value == other.value;
}

class MyStreamEntryTwinRustAsyncSse {
  final String hello;

  const MyStreamEntryTwinRustAsyncSse({
    required this.hello,
  });

  @override
  int get hashCode => hello.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MyStreamEntryTwinRustAsyncSse &&
          runtimeType == other.runtimeType &&
          hello == other.hello;
}

class MyStructContainingStreamSinkTwinRustAsyncSse {
  final int a;
  final RustStreamSink<int> b;

  const MyStructContainingStreamSinkTwinRustAsyncSse({
    required this.a,
    required this.b,
  });

  @override
  int get hashCode => a.hashCode ^ b.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is MyStructContainingStreamSinkTwinRustAsyncSse &&
          runtimeType == other.runtimeType &&
          a == other.a &&
          b == other.b;
}