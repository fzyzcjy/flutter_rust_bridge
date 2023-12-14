import 'package:flutter/material.dart';
import 'package:frb_example_gallery/src/rust/api/polars.dart';

class PolarsPageBody extends StatefulWidget {
  const PolarsPageBody({super.key});

  @override
  State<PolarsPageBody> createState() => _PolarsPageBodyState();
}

class _PolarsPageBodyState extends State<PolarsPageBody> {
  @override
  void initState() {
    super.initState();
    () async {
      // TODO support positional arguments (not hard)
      final df = await (await readSampleDataset())
          .lazy()
          .filter(predicate: col(name: "sepal_length").gt(other: lit(t: 5)))
          .groupBy(expr: col(name: "species"))
          .agg(expr: col(name: "*").sum())
          .collect();
      for (final colName in df.getColumnNames()) {
        print('colName=$colName values=${await df.getColumn(name: colName)}');
      }
    }();
  }

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        _buildDartCodeSection(),
        _buildRustCodeSection(),
        _buildInputSection(),
        _buildOutputSection(),
      ],
    );
  }

  Widget _buildDartCodeSection() {
    return Text('TODO');
  }

  Widget _buildRustCodeSection() {
    return Text('TODO');
  }

  Widget _buildInputSection() {
    return Text('TODO');
  }

  Widget _buildOutputSection() {
    return Text('TODO');
  }
}
