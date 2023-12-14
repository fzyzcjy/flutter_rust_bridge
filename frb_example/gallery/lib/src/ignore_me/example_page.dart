import 'package:flutter/material.dart';

class ExamplePage extends StatelessWidget {
  final String title;
  final String subtitle;
  final Widget icon;

  const ExamplePage({
    super.key,
    required this.title,
    required this.subtitle,
    required this.icon,
  });

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
