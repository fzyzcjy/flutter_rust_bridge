import 'dart:typed_data';

import 'package:flutter/material.dart';
import 'package:flutter_via_create/src/rust/api/ohos_smoke.dart';
import 'package:flutter_via_create/src/rust/api/simple.dart';
import 'package:flutter_via_create/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  final greeting = greet(name: 'Tom');
  final bytes = reverseBytes(value: Uint8List.fromList([1, 2, 3, 5]));
  final payload = transformPayload(
    value: OhosSmokePayload(code: 41, label: 'payload'),
  );
  final asyncGreeting = await asyncGreet(name: 'Tom');
  final successfulValue = fallibleValue(shouldFail: false);
  var failureWasReported = false;
  try {
    fallibleValue(shouldFail: true);
  } on Object {
    failureWasReported = true;
  }
  final streamedValues = <int>[];
  await for (final value in emitValues()) {
    streamedValues.add(value);
  }

  final passed =
      greeting == 'Hello, Tom!' &&
      listEquals(bytes, const [5, 3, 2, 1]) &&
      payload.code == 42 &&
      payload.label == 'payload!' &&
      asyncGreeting == 'Async hello, Tom!' &&
      successfulValue == 'smoke ok' &&
      failureWasReported &&
      listEquals(streamedValues, const [7, 11, 13]);
  debugPrint('FRB_OHOS_SMOKE_RESULT=${passed ? 'PASS' : 'FAIL'}');
  runApp(_SmokeApp(result: passed ? 'PASS' : 'FAIL'));
}

bool listEquals<T>(List<T> first, List<T> second) {
  if (first.length != second.length) return false;
  for (var index = 0; index < first.length; index++) {
    if (first[index] != second[index]) return false;
  }
  return true;
}

class _SmokeApp extends StatelessWidget {
  final String result;

  const _SmokeApp({required this.result});

  @override
  Widget build(BuildContext context) => MaterialApp(
    home: Scaffold(body: Center(child: Text('Result: `$result`'))),
  );
}
