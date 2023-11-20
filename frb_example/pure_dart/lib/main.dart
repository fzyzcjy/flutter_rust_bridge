import 'src/ffi.io.dart' if (dart.library.html) 'src/ffi.web.dart';

void main() {
  final api = initializeExternalLibrary(dylibPath);
  tearDownAll(() => api.dispose());

  test('dart call simpleAdder', () async {
    expect(await api.simpleAdder(a: 42, b: 100), 142);
  });

  // TODO
}
