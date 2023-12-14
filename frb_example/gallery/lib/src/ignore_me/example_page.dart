import 'package:flutter/material.dart';

// Usually do not use inheritance, but we are just doing a minimal demo ;)
class ExampleBasePage extends StatelessWidget {
  const ExampleBasePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text('TODO'),
      ),
      body: Text('TODO'),
    );
  }
}
