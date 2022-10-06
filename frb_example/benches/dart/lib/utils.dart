import 'dart:math' as math;

/// random string to seed benchmarks, tests, etc
String getRandomString(int length) {
  const characters = '+-*=?AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz';
  math.Random random = math.Random();
  return String.fromCharCodes(
      Iterable.generate(length, (_) => characters.codeUnitAt(random.nextInt(characters.length))));
}
