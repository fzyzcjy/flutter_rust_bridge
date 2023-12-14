import 'package:frb_example_gallery/src/rust/api/polars.dart';

typedef SimpleTable = List<List<String>>;

// TODO remove this "RwLock" prefix
Future<SimpleTable> convertToSimpleTable(RwLockDataFrame df) async {
  return _transpose([
    for (final colName in df.getColumnNames()) await df.getColumn(name: colName)
  ]);
}

SimpleTable _transpose(SimpleTable raw) {
  if (raw.isEmpty) return [];
  final ans = List<List<String>>.generate(raw[0].length, (_) => []);
  for (var colIndex = 0; colIndex < raw.length; ++colIndex) {
    for (var rowIndex = 0; rowIndex <= raw[0].length; ++rowIndex) {
      ans[rowIndex][colIndex] = raw[colIndex][rowIndex];
    }
  }
  return ans;
}
