// see https://github.com/flutter/flutter/issues/55870

import 'dart:io';

const truthyValues = ['true', '1', 'yes'];

/// ✅ correctly read e.g. export JSON=true && ...
/// ❌ fail at reading e.g. dart --define=USE_OPEN_DYLIB=true ...
/// ❌ fail at reading e.g. dart -DUSE_OPEN_DYLIB=true ...
bool? readEnvFromPlatform(String envVarName) =>
    (Platform.environment[envVarName] != null &&
            Platform.environment[envVarName]!.isNotEmpty)
        ? truthyValues.contains(Platform.environment[envVarName]!.toLowerCase())
        : null;

/// ✅ correctly read e.g. dart --define=USE_OPEN_DYLIB=true ...
/// ✅ correctly read e.g. dart -DUSE_OPEN_DYLIB=true ...
/// ❌ fail at reading e.g. export JSON=true && ...
bool? readEnvFromPrimitive(String envVarName) =>
    bool.hasEnvironment(envVarName) ? bool.fromEnvironment(envVarName) : null;

bool readBoolEnv(String envVarName, {bool defaultValue = false}) =>
    readEnvFromPrimitive(envVarName) ??
    readEnvFromPlatform(envVarName) ??
    defaultValue;

final isWeb = readBoolEnv('dart.library.html');
final int? portEnv = Platform.environment['PORT'] != null
    ? int.parse(Platform.environment['PORT']!)
    : null;
final useOpenDylib = readBoolEnv('USE_OPEN_DYLIB');
final String? dylibPath = Platform.environment['DYLIB_PATH'];
