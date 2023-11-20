import 'dart:ffi';

import 'package:flutter_rust_bridge/src/utils/dart_c_object_into_dart.dart';

import '_platform_types_io.dart';

List<dynamic> wireSyncReturnIntoDart(WireSyncReturn syncReturn) => dartCObjectIntoDart(syncReturn.ref);
