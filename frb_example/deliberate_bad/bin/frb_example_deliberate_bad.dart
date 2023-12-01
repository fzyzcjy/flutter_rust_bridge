void main(List<String> args) {
  switch (args[0]) {
    case 'Good':
      print('I am good Dart code');

    default:
      throw Exception('Unknown args $args');
  }
}
