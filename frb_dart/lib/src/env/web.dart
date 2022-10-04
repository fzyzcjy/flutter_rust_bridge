// see https://github.com/flutter/flutter/issues/55870

const isWeb = bool.fromEnvironment('dart.library.html');
final int? portEnv =
    int.fromEnvironment('PORT') != 0 ? int.fromEnvironment('PORT') : null;
const isSilicon = bool.fromEnvironment('SILICON');
const useJSON = bool.fromEnvironment('JSON');
final String? dylibPath = String.fromEnvironment('DYLIB_PATH').isNotEmpty
    ? String.fromEnvironment('DYLIB_PATH')
    : null;
