// ignore_for_file: invalid_use_of_internal_member

import '../frb_generated.dart';

Future<int> simpleAdder({required int a, required int b, dynamic hint}) async =>
    Rust.instance.dispatcher.simpleAdder(a: a, b: b, hint: hint);
