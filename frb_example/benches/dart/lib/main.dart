import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'ffi.io.dart' if (dart.library.html) 'ffi.web.dart';
import 'package:flutter_rust_bridge_benchmark/utils.dart';
import 'package:test/test.dart';
import 'package:uuid/uuid.dart';

void main(List<String> args) async {
  String dylibPath = args[0];
  print('flutter_rust_bridge example program start (dylibPath=$dylibPath)');
  print('construct api');
  final api = initializeExternalLibrary(dylibPath);

  group('benchmark simple roundtrip', () {
    late List<UuidValue> thousandUuids;
    late List<String> thousandStrings;
    late List<String> thousandUuidsAsStrings;
    late List<UuidValue> hundredThousandUuids;
    late List<String> hundredThousandStrings;
    late List<String> hundredThousandUuidsAsStrings;
    setUp(() {
      final Uuid uuid = Uuid();
      thousandUuids = List<UuidValue>.generate(1000, (index) => uuid.v4obj(),
          growable: false);
      thousandStrings = List<String>.generate(
          1000, (index) => getRandomString(uuidSizeInBytes),
          growable: false);
      thousandUuidsAsStrings =
          thousandUuids.map((e) => e.toString()).toList(growable: false);
      hundredThousandUuids = List<UuidValue>.generate(
          100000, (index) => uuid.v4obj(),
          growable: false);
      hundredThousandStrings = List<String>.generate(
          100000, (index) => getRandomString(uuidSizeInBytes),
          growable: false);
      hundredThousandUuidsAsStrings =
          hundredThousandUuids.map((e) => e.toString()).toList(growable: false);
    });
    test('1,000 uuids', () async {
      final output = await api.handleUuids(ids: thousandUuids);
      expect(output, thousandUuids);
    });
    test('1,000 strings', () async {
      final output = await api.handleStrings(strings: thousandStrings);
      expect(output, thousandStrings);
    });
    test('1,000 uuids -> 1,000 strings', () async {
      final output = await api.handleUuidsConvertToStrings(ids: thousandUuids);
      expect(output, thousandUuidsAsStrings);
    });
    test('100,000 uuids', () async {
      final output = await api.handleUuids(ids: hundredThousandUuids);
      expect(output, hundredThousandUuids);
    });
    test('100,000 strings', () async {
      final output = await api.handleStrings(strings: hundredThousandStrings);
      expect(output, hundredThousandStrings);
    });
    test('100,000 uuids -> 100,000 strings', () async {
      final output = await api.handleUuidsConvertToStrings(
        ids: hundredThousandUuids,
      );
      expect(output, hundredThousandUuidsAsStrings);
    });
  });
}

class MatchBigInt extends CustomMatcher {
  MatchBigInt(matcher)
      : super("is a numeric", "value", _featureValueOf(matcher));
  @override
  Object? featureValueOf(actual) => _featureValueOf(actual);

  static Object? _featureValueOf(actual) {
    if (actual is Iterable) return actual.map(_featureValueOf);
    if (actual is int) return BigInt.from(actual);
    return actual;
  }
}

// vim:expandtab:ts=2:sw=2
