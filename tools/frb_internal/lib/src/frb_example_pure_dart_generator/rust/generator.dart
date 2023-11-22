import 'dart:io';

void generateRust({required Uri rustRoot}) {
  void writeFile(String relativePath, String code) =>
      File(rustRoot.resolve(relativePath).toFilePath()).writeAsStringSync(code);

  writeFile('src/api/primitive.rs', _generateSrcApiPrimitive());
}

String _generateSrcApiPrimitive() {
  return TODO;
}
