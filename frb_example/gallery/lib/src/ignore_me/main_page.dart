import 'package:flutter/material.dart';
import 'package:flutter/scheduler.dart';
import 'package:frb_example_gallery/src/examples/mandelbrot.dart';
import 'package:frb_example_gallery/src/examples/polars.dart';
import 'package:frb_example_gallery/src/ignore_me/example_page.dart';

class MainPageWidget extends StatelessWidget {
  const MainPageWidget({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      theme: ThemeData(
        colorScheme: const ColorScheme.light(
          background: Colors.white,
          primary: Colors.blue,
        ),
      ),
      home: Scaffold(
        appBar: AppBar(title: const Text('Gallery')),
        body: const _MainPageWidgetInner(),
      ),
    );
  }
}

class _MainPageWidgetInner extends StatefulWidget {
  const _MainPageWidgetInner();

  @override
  State<_MainPageWidgetInner> createState() => _MainPageWidgetInnerState();
}

class _MainPageWidgetInnerState extends State<_MainPageWidgetInner> {
  @override
  void initState() {
    super.initState();

    SchedulerBinding.instance.addPostFrameCallback((_) {
      final urlGalleryPage = Uri.base.queryParameters["gallery_page"];
      final page = kPages.where((x) => x.name == urlGalleryPage).firstOrNull;
      if (page != null) _jumpPage(page);
    });
  }

  static const kPages = [
    ExamplePage(
      name: 'mandelbrot',
      title: 'Mandelbrot',
      subtitle: 'Example: Use Rust to write algorithms',
      icon: Icon(
        // Icons.query_stats_outlined,
        Icons.center_focus_strong_outlined,
        color: Colors.green,
      ),
      body: MandelbrotPageBody(),
    ),
    ExamplePage(
      name: 'polars',
      title: 'Polars',
      subtitle: 'Example: Use well-developed Rust libraries in Dart',
      icon: Icon(
        Icons.subject_outlined,
        color: Colors.blue,
      ),
      body: PolarsPageBody(),
    ),
  ];

  @override
  Widget build(BuildContext context) {
    return Center(
      child: ConstrainedBox(
        constraints: const BoxConstraints(maxWidth: 1000),
        child: Row(
          mainAxisSize: MainAxisSize.min,
          children: [
            for (final page in kPages) _buildButton(page),
          ],
        ),
      ),
    );
  }

  Widget _buildButton(ExamplePage page) {
    return Builder(
      builder: (context) => Expanded(
        child: Padding(
          padding: const EdgeInsets.symmetric(horizontal: 32),
          child: InkWell(
            borderRadius: BorderRadius.circular(8),
            onTap: () => _jumpPage(page),
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

  void _jumpPage(ExamplePage page) {
    Navigator.of(context).push(MaterialPageRoute(builder: (_) => page));
  }
}
