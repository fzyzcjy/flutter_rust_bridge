import 'dart:io';

import 'package:flutter_rust_bridge_internal/src/utils/execute_process.dart';
import 'package:flutter_rust_bridge_internal/src/utils/generator_utils.dart';

Future<void> generateRust({required Uri rustRoot}) async {
  final textOfPathMap = {
    'src/api/primitive.rs': _generateSrcApiPrimitive(),
  };

  writeCodeFiles(rustRoot, textOfPathMap);
  await executeRustFormat(workingDirectory: rustRoot.toFilePath());
}

String _generateSrcApiPrimitive() {
  return TODO;
}
