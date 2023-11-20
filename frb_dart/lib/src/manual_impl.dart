import 'package:flutter_rust_bridge/src/exceptions.dart';
import 'package:meta/meta.dart';

/// This file contains functions that are manually written, but on the other hand,
/// the kinds of such functions are usually generated from frb_codegen.

@internal
PanicException wire2apiPanicError(dynamic raw) => PanicException(raw as String);
