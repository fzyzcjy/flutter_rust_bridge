// see https://github.com/flutter/flutter/issues/55870

const useJSON = bool.fromEnvironment('JSON');
const isWeb = bool.fromEnvironment('dart.library.html');
const isSilicon = bool.fromEnvironment('SILICON');
final sampleCount = int.fromEnvironment('SAMPLE_COUNT', defaultValue: 1000);
