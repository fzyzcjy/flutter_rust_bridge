// ignore_for_file: invalid_use_of_internal_member

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Stream<String> handleStreamRealisticTwinNormal(
        {required String arg, dynamic hint}) =>
    RustLib.instance.api.handleStreamRealisticTwinNormal(arg: arg, hint: hint);
