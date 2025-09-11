import 'package:web/web.dart' as web;

void main() {
  final now = DateTime.now();
  final element = web.document.querySelector('#output') as web.HTMLDivElement;
  element.textContent =
      'The time is ${now.hour}:${now.minute} '
      'and your Dart web app is running!';
}
