/// Arguments of `loadExternalLibary`
class ExternalLibraryLoaderConfig {
  /// The file stem
  final String stem;

  /// In io (native), the directory that may find the dynamic library
  final String? ioDirectory;

  /// In Web, the prefix path for the wasm
  final String? webPrefix;

  /// Arguments of `loadExternalLibary`
  const ExternalLibraryLoaderConfig({
    required this.stem,
    required this.ioDirectory,
    required this.webPrefix,
  });
}
