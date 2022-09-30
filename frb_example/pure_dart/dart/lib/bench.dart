import 'package:benchmark/benchmark.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'ffi.io.dart' if (dart.library.html) 'ffi.web.dart';
import 'package:uuid/uuid.dart';
import 'dart:math' as math;

void main(List<String> args) async {
  String dylibPath = args[0];
  print('flutter_rust_bridge example program start (dylibPath=$dylibPath)');
  print('bench api');
  final api = initializeExternalLibrary(dylibPath);

  group('benchmarking feature (roundtrip)', () {
    const int oneThousand = 1000;
    const int oneHundredThousand = 100000;
    const int oneMillion = 1000000;
    group('lot of uuids', () {
      late List<UuidValue> oneThousandUuids;
      late List<UuidValue> oneHundredThousandUuids;
      late List<UuidValue> oneMillionUuids;
      setUp(() {
        final uuid = Uuid();
        oneThousandUuids = List<UuidValue>.generate(oneThousand, (_) => uuid.v4obj());
        oneHundredThousandUuids = List<UuidValue>.generate(oneHundredThousand, (_) => uuid.v4obj());
        oneMillionUuids = List<UuidValue>.generate(oneMillion, (_) => uuid.v4obj());
      });
      benchmark('1,000 uuids', () async {
        final _ = await api.benchHandleUuids(ids: oneThousandUuids);
      });
      benchmark('100,000 uuids', () async {
        final _ = await api.benchHandleUuids(ids: oneHundredThousandUuids);
      });
      benchmark('1,000,000 uuids', () async {
        final _ = await api.benchHandleUuids(ids: oneMillionUuids);
      });
    });
    group('lot of strings', () {
      late List<String> oneThousandStrings;
      late List<String> oneHundredThousandStrings;
      late List<String> oneMillionStrings;
      setUp(() {
        oneThousandStrings = List<String>.generate(oneThousand, (_) => getRandomString(uuidSizeInBytes));
        oneHundredThousandStrings = List<String>.generate(oneHundredThousand, (_) => getRandomString(uuidSizeInBytes));
        oneMillionStrings = List<String>.generate(oneMillion, (_) => getRandomString(uuidSizeInBytes));
      });
      benchmark('1,000 strings', () async {
        final _ = await api.benchHandleStringList(names: oneThousandStrings);
      });
      benchmark('100,000 thousand strings', () async {
        final _ = await api.benchHandleStringList(names: oneHundredThousandStrings);
      });
      benchmark('1,000,000 strings', () async {
        final _ = await api.benchHandleStringList(names: oneMillionStrings);
      });
    });
  });
}

String getRandomString(int length) {
  const characters = '+-*=?AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz';
  math.Random random = math.Random();
  return String.fromCharCodes(
      Iterable.generate(length, (_) => characters.codeUnitAt(random.nextInt(characters.length))));
}
