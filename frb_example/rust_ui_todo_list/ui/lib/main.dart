// ignore_for_file: prefer_const_constructors

import 'package:flutter/material.dart';
import 'package:frb_example_rust_ui_todo_list/src/rust/app.dart';
import 'package:frb_example_rust_ui_todo_list/src/rust/frb_generated.dart';
import 'package:styled_widget/styled_widget.dart';

void main() => runRustApp(body: body, state: RustState.new);

Widget body(RustState state) {
  return [
    SyncTextField(
      decoration:
          InputDecoration(hintText: 'Input text and enter to add a todo'),
      text: state.inputText,
      onChanged: (text) => state.inputText = text,
      onSubmitted: (_) => state.add(),
    ).padding(bottom: 8),
    ListView(children: [
      for (final item in state.filteredItems()) todoItem(state, item)
    ]).expanded(),
    [
      for (final filter in Filter.values)
        TextButton(
          onPressed: () => state.filter = filter,
          child: Text(filter.name)
              .textColor(state.filter == filter ? Colors.blue : Colors.black87),
        ),
    ].toRow(),
  ].toColumn().padding(all: 16);
}

Widget todoItem(RustState state, Item item) {
  return [
    Checkbox(
        value: item.completed, onChanged: (_) => state.toggle(id: item.id)),
    Text(item.content).expanded(),
    IconButton(
        icon: Icon(Icons.close, color: Colors.grey),
        onPressed: () => state.remove(id: item.id)),
  ].toRow();
}
