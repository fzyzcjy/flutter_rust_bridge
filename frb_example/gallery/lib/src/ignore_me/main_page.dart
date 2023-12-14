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
              _buildButton(),
              _buildButton(),
              _buildButton(),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildButton() {
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
