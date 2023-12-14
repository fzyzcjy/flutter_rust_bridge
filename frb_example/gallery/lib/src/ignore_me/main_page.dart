import 'package:flutter/material.dart';

class MainPageWidget extends StatelessWidget {
  const MainPageWidget({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: ThemeData(
        colorScheme: const ColorScheme.light(
          background: Colors.white,
        ),
      ),
      home: Scaffold(
        appBar: AppBar(title: const Text('Gallery')),
        body: Center(
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              _buildButton(
                title: 'Mandelbrot',
                subtitle: 'Use Rust to write algorithms',
              ),
              _buildButton(
                title: 'Polars',
                subtitle: 'Use Rust well-developed libraries in Dart',
              ),
              _buildButton(
                title: 'State',
                subtitle: 'State in Rust, UI in Dart',
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildButton({
    required String title,
    required String subtitle,
  }) {
    return Padding(
      padding: const EdgeInsets.symmetric(horizontal: 32),
      child: InkWell(
        borderRadius: BorderRadius.circular(8),
        onTap: () {
          // TODO
        },
        child: const Padding(
          padding: EdgeInsets.symmetric(horizontal: 32, vertical: 32),
          child: Text('TODO'),
        ),
      ),
    );
  }
}
