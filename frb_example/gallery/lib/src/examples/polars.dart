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

  double get _value => double.tryParse(_valueController.text) ?? 5.0;

  Future<void> _executeQuery() async {
    // TODO support positional arguments (not hard)
    final df = await (await readSampleDataset())
        .lazy()
        .filter(predicate: col(name: "sepal_length").gt(other: lit(t: _value)))
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
    return Column(
      children: [
        Text.rich(
          TextSpan(
            children: [
              const TextSpan(
                text: '''df.lazy()
  .filter(col("sepal_length").gt(lit(''',
              ),
              WidgetSpan(
                child: SizedBox(
                  width: 32,
                  child: TextField(
                    controller: _valueController,
                    onChanged: (_) => _executeQuery(),
                  ),
                ),
              ),
              const TextSpan(text: ''')))
  .groupBy(col("species"))
  .agg(col("*").sum())
  .collect();''')
            ],
          ),
        ),
      ],
    );
  }

  Widget _buildOutputSection() {
    return _outputTable == null
        ? const SizedBox()
        : SimpleTableWidget(data: _outputTable!);
  }
}
