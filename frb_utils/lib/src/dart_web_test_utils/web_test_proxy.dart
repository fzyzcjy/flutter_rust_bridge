import 'dart:io';

const _kLoopbackNoProxyValue = '127.0.0.1,localhost,::1';
const _kLoopbackHosts = {'127.0.0.1', 'localhost', '::1'};

String findWebTestProxyFromEnvironment(
  Uri uri,
  Map<String, String>? environment,
) {
  if (_kLoopbackHosts.contains(uri.host)) {
    return 'DIRECT';
  }

  final mergedEnvironment = environment ?? Platform.environment;
  if (_shouldBypassProxy(uri: uri, environment: mergedEnvironment)) {
    return 'DIRECT';
  }

  final proxy =
      _findProxyValue(
        uri.scheme == 'https'
            ? ['HTTPS_PROXY', 'https_proxy', 'HTTP_PROXY', 'http_proxy']
            : ['HTTP_PROXY', 'http_proxy'],
        mergedEnvironment,
      ) ??
      _findProxyValue(['ALL_PROXY', 'all_proxy'], mergedEnvironment);
  if (proxy == null) {
    return 'DIRECT';
  }

  return _proxyDirectiveFromValue(proxy);
}

Map<String, String> createWebTestBrowserEnvironment() {
  final result = Map<String, String>.from(Platform.environment);
  final noProxy = _mergeNoProxyValue(result['NO_PROXY'], result['no_proxy']);
  result['NO_PROXY'] = noProxy;
  result['no_proxy'] = noProxy;
  return result;
}

String _mergeNoProxyValue(String? upperValue, String? lowerValue) {
  final values = <String>{
    ..._splitNoProxyValue(upperValue),
    ..._splitNoProxyValue(lowerValue),
    ..._splitNoProxyValue(_kLoopbackNoProxyValue),
  };
  return values.join(',');
}

Iterable<String> _splitNoProxyValue(String? value) sync* {
  if (value == null) {
    return;
  }

  for (final item in value.split(',')) {
    final trimmed = item.trim();
    if (trimmed.isNotEmpty) {
      yield trimmed;
    }
  }
}

bool _shouldBypassProxy({
  required Uri uri,
  required Map<String, String> environment,
}) {
  final noProxy = _mergeNoProxyValue(
    environment['NO_PROXY'],
    environment['no_proxy'],
  );
  for (final item in _splitNoProxyValue(noProxy)) {
    if (_matchesNoProxyRule(host: uri.host, rule: item)) {
      return true;
    }
  }
  return false;
}

String? _findProxyValue(List<String> keys, Map<String, String> environment) {
  for (final key in keys) {
    final value = environment[key]?.trim();
    if (value != null && value.isNotEmpty) {
      return value;
    }
  }
  return null;
}

bool _matchesNoProxyRule({required String host, required String rule}) {
  final normalizedRule = rule.startsWith('.') ? rule.substring(1) : rule;
  if (normalizedRule == '*') {
    return true;
  }
  if (host == normalizedRule) {
    return true;
  }
  return host.endsWith('.$normalizedRule');
}

String _proxyDirectiveFromValue(String value) {
  final uri = Uri.tryParse(value);
  if (uri != null && uri.host.isNotEmpty) {
    final port = uri.hasPort ? ':${uri.port}' : '';
    final directive = uri.scheme.toLowerCase().startsWith('socks')
        ? 'SOCKS'
        : 'PROXY';
    return '$directive ${uri.host}$port';
  }

  return value.contains(' ') ? value : 'PROXY $value';
}
