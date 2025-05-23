import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart'
    show ExternalLibrary;
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

  test('Codegen version check', () {
    final entrypoint = _FakeBaseEntrypointWithCodegenVersion('2.0.0');

    ExternalLibrary getExternalLibrary() {
      return ExternalLibrary.process(iKnowHowToUseIt: true);
    }

    // Version does not match, will throw a [StateError].
    expectLater(
      // ignore: invalid_use_of_protected_member
      entrypoint.initImpl(
        api: _FakeApi(),
        externalLibrary: getExternalLibrary(),
        forceSameCodegenVersion: true,
      ),
      throwsA(isA<StateError>()),
    );

    // Version matched but the stem is fake, will throw an [ArgumentError].
    expectLater(
      // ignore: invalid_use_of_protected_member
      entrypoint.initImpl(
        api: _FakeApi(),
        externalLibrary: getExternalLibrary(),
        forceSameCodegenVersion: false,
      ),
      throwsA(isA<ArgumentError>()),
    );
  });
}

class _FakeBaseEntrypointWithCodegenVersion extends _FakeBaseEntrypoint {
  _FakeBaseEntrypointWithCodegenVersion(this.codegenVersion);

  @override
  final String codegenVersion;

  @override
  ExternalLibraryLoaderConfig get defaultExternalLibraryLoaderConfig =>
      const ExternalLibraryLoaderConfig(
        stem: 'fake_codegen_version',
        ioDirectory: null,
        webPrefix: null,
      );
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
