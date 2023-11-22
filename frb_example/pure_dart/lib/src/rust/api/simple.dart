// ignore_for_file: invalid_use_of_internal_member

import '../frb_generated.dart';

Future<int> simpleAdderTwinNormal(
        {required int a, required int b, dynamic hint}) =>
    RustLib.instance.api.simpleAdderTwinNormal(a: a, b: b, hint: hint);
