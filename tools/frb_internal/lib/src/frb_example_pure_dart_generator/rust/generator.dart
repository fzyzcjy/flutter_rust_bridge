import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/utils/generator_utils.dart';

void generateRust({required Uri rustRoot}) {
  final textOfPathMap = {
    'src/api/primitive.rs': _generateSrcApiPrimitive(),
  };

  writeCodeFiles(rustRoot, textOfPathMap);
}

String _generateSrcApiPrimitive() {
  return TODO;
}
