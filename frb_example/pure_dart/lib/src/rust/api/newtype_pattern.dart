// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<NewTypeInt> handleNewtype({required NewTypeInt arg, dynamic hint}) =>
    RustLib.instance.api.handleNewtype(arg: arg, hint: hint);

class NewTypeInt {
  final BigInt field0;

  const NewTypeInt({
    required this.field0,
  });

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is NewTypeInt &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}
