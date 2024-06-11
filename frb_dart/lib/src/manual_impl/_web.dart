import 'package:flutter_rust_bridge/src/platform_types/_web.dart';
import 'package:flutter_rust_bridge/src/platform_utils/_web.dart';

/// {@macro flutter_rust_bridge.internal}
List<dynamic> wireSyncRust2DartDcoIntoDart(WireSyncRust2DartDco syncReturn) =>
    syncReturn;

/// {@macro flutter_rust_bridge.only_for_generated_code}
BigInt dcoDecodeI64(dynamic raw) => jsBigIntToDartBigInt(raw);

/// {@macro flutter_rust_bridge.only_for_generated_code}
BigInt dcoDecodeU64(dynamic raw) => jsBigIntToDartBigInt(raw);

/// {@macro flutter_rust_bridge.only_for_generated_code}
BigInt sseEncodeCastedPrimitiveI64(int raw) => BigInt.from(raw);

/// {@macro flutter_rust_bridge.only_for_generated_code}
BigInt sseEncodeCastedPrimitiveU64(int raw) => BigInt.from(raw);
