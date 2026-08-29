@TestOn('browser')
import 'package:flutter_rust_bridge/src/exceptions.dart';
import 'package:flutter_rust_bridge/src/generalized_typed_data/generalized_typed_data.dart';
import 'package:test/test.dart';

void main() {
  test(
    'web 64-bit typed lists preserve BigInt values and reject invalid writes',
    () {
      final signed = Int64List.fromList([1, -2]);
      final unsigned = Uint64List.fromList([3]);

      signed[1] = BigInt.parse('-9223372036854775808');
      unsigned[0] = BigInt.parse('18446744073709551615');

      expect(signed, [BigInt.one, BigInt.parse('-9223372036854775808')]);
      expect(unsigned, [BigInt.parse('18446744073709551615')]);
      expect(() => signed[0] = 'not an integer', throwsArgumentError);
      expect(
        () => signed.length = 3,
        throwsA(isA<UnmodifiableTypedListException>()),
      );
    },
  );
}
