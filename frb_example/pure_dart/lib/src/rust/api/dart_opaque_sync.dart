// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Object syncLoopback({required Object opaque, dynamic hint}) =>
    RustLib.instance.api.syncLoopback(opaque: opaque, hint: hint);

Object? syncOptionLoopback({Object? opaque, dynamic hint}) =>
    RustLib.instance.api.syncOptionLoopback(opaque: opaque, hint: hint);

String syncAcceptDartOpaque({required Object opaque, dynamic hint}) =>
    RustLib.instance.api.syncAcceptDartOpaque(opaque: opaque, hint: hint);

/// [DartWrapObject] can be safely retrieved on a dart thread.
String unwrapDartOpaque({required Object opaque, dynamic hint}) =>
    RustLib.instance.api.unwrapDartOpaque(opaque: opaque, hint: hint);

Object returnNonDroppableDartOpaque({required Object opaque, dynamic hint}) =>
    RustLib.instance.api
        .returnNonDroppableDartOpaque(opaque: opaque, hint: hint);
