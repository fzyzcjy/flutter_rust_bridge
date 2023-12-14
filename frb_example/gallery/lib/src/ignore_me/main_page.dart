import 'package:flutter/material.dart';
import 'package:frb_example_gallery/src/examples/mandelbrot.dart';
import 'package:frb_example_gallery/src/examples/polars.dart';
import 'package:frb_example_gallery/src/examples/state.dart';
import 'package:frb_example_gallery/src/ignore_me/example_page.dart';

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
                  page: const ExamplePage(
                    title: 'Mandelbrot',
                    subtitle: 'Example: Use Rust to write algorithms',
                    icon: Icon(
                      // Icons.query_stats_outlined,
                      Icons.center_focus_strong_outlined,
                      color: Colors.green,
                    ),
                    body: MandelbrotPageBody(),
                  ),
                ),
                _buildButton(
                  page: const ExamplePage(
                    title: 'Polars',
                    subtitle:
                        'Example: Use well-developed Rust libraries in Dart',
                    icon: Icon(
                      Icons.folder_outlined,
                      color: Colors.blue,
                    ),
                    body: PolarsPageBody(),
                  ),
                ),
                // _buildButton(
                //   page: const ExamplePage(
                //     title: 'State',
                //     subtitle: 'Example: State in Rust, UI in Dart',
                //     icon: Icon(
                //       Icons.article_outlined,
                //       color: Colors.cyan,
                //     ),
                //     body: StatePageBody(),
                //   ),
                // ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildButton({
    required ExamplePage page,
  }) {
    return Builder(
      builder: (context) => Expanded(
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 32),
          child: InkWell(
            borderRadius: BorderRadius.circular(8),
            onTap: () => Navigator.of(context)
                .push(MaterialPageRoute(builder: (_) => page)),
            child: Padding(
              padding: const EdgeInsets.symmetric(horizontal: 32, vertical: 32),
              child: Column(
                mainAxisSize: MainAxisSize.min,
                children: [
                  IconTheme.merge(
                    data: const IconThemeData(size: 64),
                    child: page.icon,
                  ),
                  const SizedBox(height: 8),
                  Text(
                    page.title,
                    style: const TextStyle(fontSize: 20),
                  ),
                  const SizedBox(height: 8),
                  SizedBox(
                    height: 64,
                    child: Text(
                      page.subtitle,
                      textAlign: TextAlign.center,
                      style: const TextStyle(
                        fontSize: 14,
                      ),
                    ),
                  ),
                ],
              ),
            ),
          ),
        ),
      ),
    );
  }
}
