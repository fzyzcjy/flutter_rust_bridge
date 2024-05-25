import 'package:flutter_rust_bridge/src/platform_types/_web.dart';
import 'package:flutter_rust_bridge/src/platform_utils/_web.dart';

/// {@macro flutter_rust_bridge.internal}
List<dynamic> wireSyncRust2DartDcoIntoDart(WireSyncRust2DartDco syncReturn) =>
    syncReturn;

/// {@macro flutter_rust_bridge.only_for_generated_code}
BigInt dcoDecodeI64(dynamic raw) => jsBigIntToDartBigInt(raw);

/// {@macro flutter_rust_bridge.only_for_generated_code}
BigInt dcoDecodeU64(dynamic raw) => jsBigIntToDartBigInt(raw);
