import 'package:flutter/material.dart';
import 'package:frb_example_rust_ui_counter/src/rust/app.dart';
import 'package:frb_example_rust_ui_counter/src/rust/frb_generated.dart';
import 'package:styled_widget/styled_widget.dart';

void main() => runRustApp(body: body, state: RustState.new);

Widget body(RustState state) {
  return [
    Text('Count: ${state.count}'),
    TextButton(onPressed: state.increment, child: Text('+1')),
  ].toColumn().padding(all: 16);
}
