// see https://github.com/flutter/flutter/issues/55870

import 'dart:io';

bool readEnv(String envVarName, {bool defaultValue = false}) =>
    (Platform.environment[envVarName] ?? defaultValue.toString()) == 'true';

final isWeb = readEnv('dart.library.html');
final int? portEnv = Platform.environment['PORT'] != null
    ? int.parse(Platform.environment['PORT']!)
    : null;
final isSilicon = readEnv('SILICON');
final useJSON = readEnv('JSON');
final String? dylibPath = Platform.environment['DYLIB_PATH'];
