import 'package:flutter/material.dart';
import 'package:frb_example_gallery/src/ignore_me/main_page.dart';
import 'package:frb_example_gallery/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return const MainPageWidget();
  }
}
