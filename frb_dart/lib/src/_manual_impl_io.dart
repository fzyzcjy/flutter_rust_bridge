import 'dart:ffi';

import '_platform_types_io.dart';

List<dynamic> wireSyncReturnIntoDart(WireSyncReturn syncReturn) => syncReturn.ref.intoDart();
