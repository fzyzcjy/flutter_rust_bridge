import 'dart:ffi';
import 'dart:io';

const dylibSource = 'DYNAMIC_LIBRARY_SOURCE';

enum DylibSourceKind {
  executable,
  process,
  open;

  static DylibSourceKind? fromEnvironment(String value) {
    String source = String.fromEnvironment(dylibSource).trim();
    if (source.isEmpty) return null;
    if (!['executable', 'process', 'open'].contains(source)) {
      print(
          "warning: invalid $dylibSource '$source' ignored (expected 'executable', 'process' or 'open')");
    } else {
      print('info: use $dylibSource=$source to open dynamic library');
    }
    switch (source) {
      case 'executable':
        return DylibSourceKind.executable;
      case 'process':
        return DylibSourceKind.process;
      case 'open':
        return DylibSourceKind.open;
      default:
        return null;
    }
  }
}

enum LanguageExecutionContext {
  dart,
  flutter;
}

/// open dynamic library
/// if user defines a `DYNAMIC_LIBRARY_SOURCE` for Dart in CLI it takes precedence,
/// otherwise fallback to default dynamic library opening mode
DynamicLibrary open(
    {DylibSourceKind? maybeUserDefinedKind,
    required String path,
    required LanguageExecutionContext ctx}) {
  switch (maybeUserDefinedKind) {
    case DylibSourceKind.executable:
      return DynamicLibrary.executable();
    case DylibSourceKind.process:
      return DynamicLibrary.process();
    case DylibSourceKind.open:
      return DynamicLibrary.open(path);
    default:
      return ctx == LanguageExecutionContext.dart
          ? (Platform.isMacOS || Platform.isIOS)
              ? DynamicLibrary.executable()
              : DynamicLibrary.open(path)
          : Platform.isIOS
              ? DynamicLibrary.process()
              : Platform.isMacOS
                  ? DynamicLibrary.executable()
                  : DynamicLibrary.open(path);
  }
}
