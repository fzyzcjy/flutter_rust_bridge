import 'dart:ffi';

import 'package:flutter_rust_bridge/src/platform_types/_io.dart';
import 'package:flutter_rust_bridge/src/utils/dart_c_object_into_dart.dart';

List<dynamic> wireSyncReturnIntoDart(WireSyncReturn syncReturn) => dartCObjectIntoDart(syncReturn.ref);
