import 'package:flutter/material.dart';
import 'package:flutter_via_create/src/rust/api/simple.dart';
import 'package:flutter_via_create/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  final result = greet(name: 'Tom');
  final smokeStatus = await runFrbSmokeSuite();
  debugPrint('FRB_OHOS_SMOKE_RESULT=$smokeStatus');
  runApp(MyApp(result: result, smokeStatus: smokeStatus));
}

Future<String> runFrbSmokeSuite() async {
  final asyncResult = await smokeAsync(input: 41);
  if (asyncResult != 42) {
    throw StateError('Unexpected async result: $asyncResult');
  }

  final streamResult = await smokeStream(count: 3).toList();
  if (streamResult.join(',') != '0,1,2') {
    throw StateError('Unexpected stream result: $streamResult');
  }

  String? callbackInput;
  final callbackResult = await smokeCallback(
    callback: (value) {
      callbackInput = value;
      return '$value-dart';
    },
  );
  if (callbackInput != 'rust' || callbackResult != 'rust-dart') {
    throw StateError(
      'Unexpected callback result: input=$callbackInput, output=$callbackResult',
    );
  }

  final counter = await smokeCounterCreate(initial: 10);
  final addedValue = await counter.add(delta: 5);
  final counterValue = await counter.value();
  counter.dispose();
  if (addedValue != 15 || counterValue != 15) {
    throw StateError(
      'Unexpected opaque result: add=$addedValue, value=$counterValue',
    );
  }

  var errorConverted = false;
  try {
    await smokeError(shouldFail: true);
  } catch (error) {
    errorConverted = error.toString().contains('deliberate OHOS smoke error');
  }
  if (!errorConverted) {
    throw StateError('Rust error was not converted to a diagnostic Dart error');
  }

  final payload = await smokePayload(size: 65536);
  if (payload.label != 'payload' ||
      payload.bytes.length != 65536 ||
      payload.bytes.first != 0 ||
      payload.bytes.last != 24 ||
      payload.state != SmokeState.ready) {
    throw StateError('Unexpected struct/enum/byte payload');
  }

  return 'PASS';
}

class MyApp extends StatelessWidget {
  final String result;
  final String smokeStatus;

  const MyApp({required this.result, required this.smokeStatus, super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Center(
          child: Text(
            'Action: Call Rust `greet("Tom")`\n'
            'Result: `$result`\n'
            'Extended smoke: `$smokeStatus`',
          ),
        ),
      ),
    );
  }
}
