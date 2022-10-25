// see https://github.com/flutter/flutter/issues/55870

const isWeb = bool.fromEnvironment('dart.library.html');
final int? portEnv =
    const int.fromEnvironment('PORT') != 0 ? int.fromEnvironment('PORT') : null;
const useOpenDylib = bool.fromEnvironment('USE_OPEN_DYLIB');
