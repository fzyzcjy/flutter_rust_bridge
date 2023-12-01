// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<void> makeStackBufferOverflow({dynamic hint}) =>
    RustLib.instance.api.makeStackBufferOverflow(hint: hint);

Future<void> makeHeapUseAfterFree({dynamic hint}) =>
    RustLib.instance.api.makeHeapUseAfterFree(hint: hint);
