// ignore_for_file: invalid_use_of_internal_member

import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';

Future<int> simpleAdder({required int a, required int b, dynamic hint}) async =>
    FrbExamplePureDart.instance.dispatcher.simpleAdder(a: a, b: b, hint: hint);
