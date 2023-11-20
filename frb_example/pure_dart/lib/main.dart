import 'src/ffi.io.dart' if (dart.library.html) 'src/ffi.web.dart';

void main() {
  final api = initializeExternalLibrary(dylibPath);
  tearDownAll(() => api.dispose());

  // TODO
}
