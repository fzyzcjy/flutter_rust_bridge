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
  final numRows = raw[0].length;
  final ans = List<List<String>>.generate(numRows, (_) => []);
  for (var colIndex = 0; colIndex < raw.length; ++colIndex) {
    for (var rowIndex = 0; rowIndex < numRows; ++rowIndex) {
      ans[rowIndex].add(raw[colIndex][rowIndex]);
    }
  }
  return ans;
}
