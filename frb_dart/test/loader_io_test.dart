@TestOn('vm')
import 'dart:io';

import 'package:flutter_rust_bridge/src/loader/_io.dart';
import 'package:test/test.dart';

void main() {
  late Directory dir;
  late String libPath;
  final attempts = <String>[];

  String openPackaged(String debugInfo) {
    attempts.add('packaged $debugInfo');
    return 'packaged';
  }

  String failPackaged(String debugInfo) {
    attempts.add('packaged $debugInfo');
    throw ArgumentError('missing packaged');
  }

  String openPath(String path, String debugInfo) {
    attempts.add('path $path $debugInfo');
    return 'path';
  }

  setUp(() {
    attempts.clear();
    dir = Directory.systemTemp.createTempSync('frb_loader_test');
    libPath = dir.uri.resolve('libfoo.so').toFilePath();
    File(libPath).createSync();
  });

  tearDown(() => dir.deleteSync(recursive: true));

  test('packaged library wins over an existing non-packaged one', () {
    final result = openPreferringPackaged(
      name: 'libfoo.so',
      overrideNativeLibDir: null,
      nativeLibDirWhenNonPackaged: dir.uri,
      openPackaged: openPackaged,
      openPath: openPath,
    );

    expect(result, 'packaged');
    expect(attempts, ['packaged ']);
  });

  test('falls back to non-packaged library when packaged fails to open', () {
    final result = openPreferringPackaged(
      name: 'libfoo.so',
      overrideNativeLibDir: null,
      nativeLibDirWhenNonPackaged: dir.uri,
      openPackaged: failPackaged,
      openPath: openPath,
    );

    expect(result, 'path');
    expect(attempts, [
      'packaged ',
      'path $libPath (after failing to open packaged library: '
          'Invalid argument(s): missing packaged)',
    ]);
  });

  test('rethrows the packaged error when there is nothing to fall back to', () {
    for (final nonPackagedDir in [null, dir.uri.resolve('missing/')]) {
      expect(
        () => openPreferringPackaged(
          name: 'libfoo.so',
          overrideNativeLibDir: null,
          nativeLibDirWhenNonPackaged: nonPackagedDir,
          openPackaged: failPackaged,
          openPath: openPath,
        ),
        throwsArgumentError,
      );
    }
    expect(
      () => openPreferringPackaged(
        name: 'libfoo.so',
        overrideNativeLibDir: null,
        nativeLibDirWhenNonPackaged: dir.uri,
        openPackaged: (_) => throw StateError('unexpected'),
        openPath: openPath,
      ),
      throwsStateError,
    );
    expect(attempts.where((a) => a.startsWith('path')), isEmpty);
  });

  test('override directory wins over packaged library when it has the file', () {
    final missingDir = dir.uri.resolve('missing/');
    expect(
      openPreferringPackaged(
        name: 'libfoo.so',
        overrideNativeLibDir: dir.uri,
        nativeLibDirWhenNonPackaged: null,
        openPackaged: openPackaged,
        openPath: openPath,
      ),
      'path',
    );
    expect(
      openPreferringPackaged(
        name: 'libfoo.so',
        overrideNativeLibDir: missingDir,
        nativeLibDirWhenNonPackaged: dir.uri,
        openPackaged: openPackaged,
        openPath: openPath,
      ),
      'packaged',
    );
    expect(attempts, [
      'path $libPath ',
      'packaged (after trying ${missingDir.resolve('libfoo.so').toFilePath()} '
          'but it does not exist)',
    ]);
  });
}
