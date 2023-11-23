// ignore_for_file: invalid_use_of_internal_member

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

int funcReturnErrorTwinSync({dynamic hint}) =>
    RustLib.instance.api.funcReturnErrorTwinSync(hint: hint);

int funcReturnPanicTwinSync({dynamic hint}) =>
    RustLib.instance.api.funcReturnPanicTwinSync(hint: hint);
