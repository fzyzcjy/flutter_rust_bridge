@TestOn('vm')
import 'dart:async';
import 'dart:convert';
import 'dart:io';

import 'package:flutter_rust_bridge/src/cli/cli_utils.dart';
import 'package:test/test.dart';

void main() {
  late IOSink oldCliStderr;
  late Never Function(int) oldCliExit;
  late StreamController<List<int>> stderrController;
  late StringBuffer stderrBuffer;
  late IOSink testCliStderr;

  setUp(() {
    oldCliStderr = cliStderr;
    oldCliExit = cliExit;
    stderrController = StreamController<List<int>>();
    stderrBuffer = StringBuffer();
    stderrController.stream.transform(utf8.decoder).listen(stderrBuffer.write);
    testCliStderr = IOSink(stderrController.sink);
    cliStderr = testCliStderr;
    cliExit = (code) => throw ExitIntercepted(code);
  });

  tearDown(() async {
    cliStderr = oldCliStderr;
    cliExit = oldCliExit;
    await testCliStderr.flush();
    await testCliStderr.close();
    await stderrController.close();
  });

  test('err returns the original message', () {
    expect(err('problem'), 'problem');
  });

  test('eprint writes formatted message to cliStderr', () async {
    eprint('problem');
    await testCliStderr.flush();
    expect(stderrBuffer.toString(), 'error: problem\n');
  });

  test('bail prints and exits through injected handler', () async {
    expect(
      () => bail('fatal'),
      throwsA(isA<ExitIntercepted>().having((e) => e.code, 'code', 1)),
    );
    await testCliStderr.flush();
    expect(stderrBuffer.toString(), 'error: fatal\n');
  });

  test('findDartPackageDirectory walks upward to pubspec', () async {
    final root = await Directory.systemTemp.createTemp('cli_utils');
    addTearDown(() async => await root.delete(recursive: true));
    await File('${root.path}/pubspec.yaml').writeAsString('name: demo\n');
    final nested = Directory('${root.path}/a/b/c')..createSync(recursive: true);

    final result = await findDartPackageDirectory(nested.path);

    expect(result, root.absolute.path);
  });

  test('findDartPackageDirectory throws when no pubspec exists', () async {
    final root = await Directory.systemTemp.createTemp('cli_utils_missing');
    addTearDown(() async => await root.delete(recursive: true));

    await expectLater(
      () => findDartPackageDirectory(root.path),
      throwsA(
        isA<ArgumentError>().having(
          (e) => e.message,
          'message',
          contains('Cannot find dart package directory'),
        ),
      ),
    );
  });
}

class ExitIntercepted implements Exception {
  final int code;

  const ExitIntercepted(this.code);
}
