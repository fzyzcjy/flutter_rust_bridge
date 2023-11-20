import 'dart:async';

import 'package:flutter_rust_bridge/src/generalized_isolate/generalized_isolate.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';

import 'io.dart' if (dart.library.html) 'web.dart' show DartPostCObject;

export 'io.dart' if (dart.library.html) 'web.dart' show ExternalLibrary, DartApiDl;

int castInt(Object? value) => value as int;
