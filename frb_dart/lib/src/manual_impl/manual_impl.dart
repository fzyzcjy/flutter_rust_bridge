/// This file contains functions that are manually written, but on the other hand,
/// the kinds of such functions are usually generated from frb_codegen.
library;

export '_common.dart';
export '_io.dart' if (dart.library.html) '_web.dart';
