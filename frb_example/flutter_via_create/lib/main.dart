import 'package:flutter/material.dart';
import 'package:flutter_via_create/src/rust/api/simple.dart';
import 'package:flutter_via_create/src/rust/frb_generated.dart';

Future<void> main() async {
  await RustLib.init();
  runApp(const MyApp());
}

Future<String?> _loadTestData() async {
  try {
    final jsonString = '{"weeks": 10}';
    final trainingPlan = TrainingPlan.testDeserialize(content: jsonString);
    return "success";
  } catch (e) {
    print('Error loading test data: $e');
    return null;
  }
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      home: FutureBuilder<String?>(
        future: _loadTestData(),
        builder: (context, snapshot) {
          if (snapshot.connectionState == ConnectionState.waiting) {
            return const Center(child: CircularProgressIndicator());
          } else if (snapshot.hasError || !snapshot.hasData) {
            return const Center(child: Text('Error loading test data'));
          } else {
            return Center(child: Text(snapshot.data ?? "no title"));
          }
        },
      ),
    );
  }
}
