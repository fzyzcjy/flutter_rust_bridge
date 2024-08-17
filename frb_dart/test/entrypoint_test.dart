import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_common.dart';
import 'package:test/test.dart';

void main() {
  test('Should be ready when initMock is called', () async {
    final entrypoint = _FakeBaseEntrypoint();

    // ignore: invalid_use_of_protected_member
    entrypoint.initMockImpl(api: _FakeApi());

    expect(entrypoint.initialized, true);
    expect(entrypoint.api, isA<_FakeApi>());
  });
}

class _FakeBaseEntrypoint extends BaseEntrypoint {
  // We do not care about these functions in this test (and they should not be called as well)
  // frb-coverage:ignore-start
  @override
  get apiImplConstructor => throw UnimplementedError();

  @override
  String get codegenVersion => throw UnimplementedError();

  @override
  get defaultExternalLibraryLoaderConfig => throw UnimplementedError();

  @override
  Future<void> executeRustInitializers() => throw UnimplementedError();

  @override
  int get rustContentHash => throw UnimplementedError();

  @override
  get wireConstructor => throw UnimplementedError();
  // frb-coverage:ignore-end
}

class _FakeApi implements BaseApi {}
