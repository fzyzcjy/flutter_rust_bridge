import 'package:flutter_rust_bridge/src/platform_types/_web.dart';

/// {@macro flutter_rust_bridge.internal}
List<dynamic> wireSyncReturnIntoDart(WireSyncReturn syncReturn) => syncReturn;

/// {@macro flutter_rust_bridge.only_for_generated_code}
// TODO is this impl ok?
BigInt wire2apiI64OrU64(dynamic raw) => BigInt.from(raw);
