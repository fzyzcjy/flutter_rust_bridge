import 'package:flutter/material.dart';
import 'package:frb_example_gallery/src/ignore_me/polars_related.dart';
import 'package:frb_example_gallery/src/rust/api/polars.dart';

class PolarsPageBody extends StatefulWidget {
  const PolarsPageBody({super.key});

  @override
  State<PolarsPageBody> createState() => _PolarsPageBodyState();
}

class _PolarsPageBodyState extends State<PolarsPageBody> {
  final _valueController = TextEditingController(text: '5');
  SimpleTable? _outputTable;

  @override
  void initState() {
    super.initState();
    _executeQuery();
  }

  Future<void> _executeQuery() async {
    // TODO support positional arguments (not hard)
    final df = await (await readSampleDataset())
        .lazy()
        .filter(
            predicate: col(name: "sepal_length").gt(
                other: lit(t: double.tryParse(_valueController.text) ?? 5.0)))
        .groupBy(expr: col(name: "species"))
        .agg(expr: col(name: "*").sum())
        .collect();
    final table = await convertToSimpleTable(df);
    setState(() => _outputTable = table);
  }

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        _buildDartCodeSection(),
        _buildOutputSection(),
      ],
    );
  }

  Widget _buildDartCodeSection() {
    return SizedBox(
      width: 64,
      child: TextField(
        controller: _valueController,
        onChanged: (_) => _executeQuery(),
      ),
    );
  }

  Widget _buildOutputSection() {
    return _outputTable == null
        ? const SizedBox()
        : SimpleTableWidget(data: _outputTable!);
  }
}
