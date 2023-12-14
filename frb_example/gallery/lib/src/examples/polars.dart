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
      final msg = await helloPolars();
      print('helloPolars=$msg');
    }();
  }

  @override
  Widget build(BuildContext context) {
    return Text('hi');
  }
}
