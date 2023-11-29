// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

String greet({required String name, dynamic hint}) =>
    RustLib.instance.api.greet(name: name, hint: hint);
