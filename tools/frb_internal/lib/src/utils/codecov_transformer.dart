// format: https://docs.codecov.com/docs/codecov-custom-coverage-format
Future<Map<String, dynamic>> transformCodecovReport(Map<String, dynamic> raw) async {
  return {
    'coverage': (raw['coverage'] as Map<String, dynamic>)
        .map((filename, data) => MapEntry(filename, _transformFile(filename, data))),
  };
}

Map<String, dynamic> _transformFile(String filename, Map<String, dynamic> data) {
  return TODO;
}
