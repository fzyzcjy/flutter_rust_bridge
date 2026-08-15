import 'package:flutter/material.dart';
import 'package:REPLACE_ME_DART_PACKAGE_NAME/src/rust/api/simple.dart';
import 'package:REPLACE_ME_DART_PACKAGE_NAME/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  final result = greet(name: 'Tom');
  debugPrint(
    'FRB_OHOS_SMOKE_RESULT=${result == 'Hello, Tom!' ? 'PASS' : 'FAIL'}',
  );
  runApp(_SmokeApp(result: result));
}

class _SmokeApp extends StatelessWidget {
  final String result;

  const _SmokeApp({required this.result});

  @override
  Widget build(BuildContext context) => MaterialApp(
    home: Scaffold(body: Center(child: Text('Result: `$result`'))),
  );
}
