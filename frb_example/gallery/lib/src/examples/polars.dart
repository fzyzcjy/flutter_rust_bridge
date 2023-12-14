import 'package:flutter/material.dart';
import 'package:frb_example_gallery/src/ignore_me/polars_related.dart';
import 'package:frb_example_gallery/src/rust/api/polars.dart';

class PolarsPageBody extends StatefulWidget {
  const PolarsPageBody({super.key});

  @override
  State<PolarsPageBody> createState() => _PolarsPageBodyState();
}

class _PolarsPageBodyState extends State<PolarsPageBody> {
  var _value = 5.0;
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
        .filter(predicate: col(name: "sepal_length").gt(other: lit(t: _value)))
        .groupBy(expr: col(name: "species"))
        .agg(expr: col(name: "*").sum())
        .collect();
    final table = await convertToSimpleTable(df);
    setState(() => _outputTable = table);
  }

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        children: [
          Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              const SizedBox(width: 32),
              const Text('Input'),
              const SizedBox(width: 24),
              Text(_value.toStringAsFixed(1)),
              SizedBox(
                width: 200,
                child: Slider(
                  value: _value,
                  onChanged: (newValue) => setState(() => _value = newValue),
                  onChangeEnd: (_) => _executeQuery(),
                  min: 0.1,
                  max: 7.0,
                  inactiveColor: Colors.blue.shade100,
                ),
              ),
            ],
          ),
          _buildOutputSection(),
        ],
      ),
    );
  }

  Widget _buildOutputSection() {
    return _outputTable == null
        ? const SizedBox()
        : SimpleTableWidget(data: _outputTable!);
  }
}
