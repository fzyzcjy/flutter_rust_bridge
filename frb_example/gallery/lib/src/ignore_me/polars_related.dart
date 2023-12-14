import 'package:flutter/material.dart';
import 'package:frb_example_gallery/src/rust/api/polars.dart';

class SimpleTable {
  final List<String> names;
  final List<List<String>> data;

  const SimpleTable({required this.names, required this.data});
}

// TODO remove this "RwLock" prefix
Future<SimpleTable> convertToSimpleTable(RwLockDataFrame df) async {
  final columnNames = df.getColumnNames();
  return SimpleTable(
    names: columnNames,
    data: _transpose(
        [for (final colName in columnNames) await df.getColumn(name: colName)]),
  );
}

List<List<String>> _transpose(List<List<String>> raw) {
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

class SimpleTableWidget extends StatelessWidget {
  final SimpleTable data;

  const SimpleTableWidget({super.key, required this.data});

  @override
  Widget build(BuildContext context) {
    return DataTable(
      columns: <DataColumn>[
        for (final name in data.names) DataColumn(label: Text(name)),
      ],
      rows: <DataRow>[
        for (final row in data.data)
          DataRow(
            cells: <DataCell>[
              for (final value in row) DataCell(Text(value)),
            ],
          ),
      ],
    );
  }
}
