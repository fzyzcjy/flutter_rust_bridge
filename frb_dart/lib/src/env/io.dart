// see https://github.com/flutter/flutter/issues/55870

import 'dart:io';

const truthyValues = ['true', '1', 'yes'];

/// ✅ correctly read e.g. export JSON=true && ...
/// ❌ fail at reading e.g. dart --define=SILICON=true ...
/// ❌ fail at reading e.g. dart -DSILICON=true ...
bool? readEnvFromPlatform(String envVarName) =>
(Platform.environment[envVarName] != null && Platform.environment[envVarName]!.isNotEmpty) ? 
    truthyValues.contains(Platform.environment[envVarName]!.toLowerCase()) : null;

/// ✅ correctly read e.g. dart --define=SILICON=true ...
/// ✅ correctly read e.g. dart -DSILICON=true ...
/// ❌ fail at reading e.g. export JSON=true && ...
bool? readEnvFromPrimitive(String envVarName) =>
    bool.hasEnvironment(envVarName) ? bool.fromEnvironment(envVarName) : null;

bool readBoolEnv(String envVarName, {bool defaultValue = false}) =>
    readEnvFromPlatform(envVarName) ??
    readEnvFromPrimitive(envVarName) ??
    defaultValue;

final isWeb = readBoolEnv('dart.library.html');
final int? portEnv = Platform.environment['PORT'] != null
    ? int.parse(Platform.environment['PORT']!)
    : null;
final isSilicon = readBoolEnv('SILICON');
final useJSON = readBoolEnv('JSON');
final String? dylibPath = Platform.environment['DYLIB_PATH'];

bool? readPlatform(String envVarName) =>
    Platform.environment[envVarName] == 'true';
bool? readViaType(String envVarName) => bool.fromEnvironment(envVarName);
