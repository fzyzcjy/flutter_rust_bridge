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
          child: ConstrainedBox(
            constraints: const BoxConstraints(maxWidth: 1000),
            child: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                _buildButton(
                  title: 'Polars',
                  subtitle:
                      'Example: Use well-developed Rust libraries in Dart',
                  icon: const Icon(
                    Icons.folder_outlined,
                    color: Colors.blue,
                  ),
                ),
                _buildButton(
                  title: 'Mandelbrot',
                  subtitle: 'Example: Use Rust to write algorithms',
                  icon: const Icon(
                    // Icons.query_stats_outlined,
                    Icons.center_focus_strong_outlined,
                    color: Colors.green,
                  ),
                ),
                _buildButton(
                  title: 'State',
                  subtitle: 'Example: State in Rust, UI in Dart',
                  icon: const Icon(
                    Icons.article_outlined,
                    color: Colors.cyan,
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildButton({
    required String title,
    required String subtitle,
    required Widget icon,
  }) {
    return Expanded(
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 32),
        child: InkWell(
          borderRadius: BorderRadius.circular(8),
          onTap: () {
            // TODO
          },
          child: Padding(
            padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 32),
            child: Column(
              mainAxisSize: MainAxisSize.min,
              children: [
                IconTheme.merge(
                  data: const IconThemeData(size: 64),
                  child: icon,
                ),
                const SizedBox(height: 8),
                Text(
                  title,
                  style: const TextStyle(fontSize: 20),
                ),
                const SizedBox(height: 8),
                SizedBox(
                  height: 64,
                  child: Text(
                    subtitle,
                    textAlign: TextAlign.center,
                  ),
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}
