import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:flutter_via_create/src/rust/api/simple.dart';
import 'package:flutter_via_create/src/rust/frb_generated.dart';

Future<void> main() async {
  // TODO temp
  FlutterError.onError = (details) {
    FlutterError.presentError(details);
    print('FlutterError.onError $details');
  };
  PlatformDispatcher.instance.onError = (error, stack) {
    print('PlatformDispatcher.instance.onError $error $stack');
    return true;
  };
  await runZonedGuarded(
    () async {
      await RustLib.init();
      runApp(const MyApp());
    },
    (error, stackTrace) => print('runZonedGuarded error $error $stackTrace'),
  );
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: Scaffold(
        appBar: AppBar(title: const Text('flutter_rust_bridge quickstart')),
        body: Center(
          child: Text(
              'Action: Call Rust `greet("Tom")`\nResult: `${greet(name: "Tom")}`'),
        ),
      ),
    );
  }
}
