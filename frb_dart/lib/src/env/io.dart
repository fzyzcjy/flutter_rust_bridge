// see https://github.com/flutter/flutter/issues/55870

import 'dart:io';

bool _check(String envVarName, {bool defaultValue = false}) =>
    (Platform.environment[envVarName] ?? defaultValue.toString()) == 'true';

final useJSON = _check('JSON');
final isWeb = _check('dart.library.html');
final isSilicon = _check('SILICON');
final sampleCount =
    int.parse(Platform.environment['SAMPLE_COUNT'] ?? 1000.toString());
