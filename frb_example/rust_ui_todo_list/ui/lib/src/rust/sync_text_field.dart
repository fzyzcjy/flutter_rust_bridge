import 'package:flutter/material.dart';

/// Adapted from https://github.com/mobxjs/mobx.dart/issues/750
class SyncTextField extends StatefulWidget {
  final String text;

  final ValueChanged<String>? onChanged;
  final InputDecoration? decoration;
  final ValueChanged<String>? onSubmitted;

  const SyncTextField({
    super.key,
    required this.text,
    this.onChanged,
    this.decoration,
    this.onSubmitted,
  });

  @override
  State<SyncTextField> createState() => _SyncTextFieldState();
}

class _SyncTextFieldState extends State<SyncTextField> {
  late final TextEditingController _controller;

  @override
  void initState() {
    super.initState();
    _controller = TextEditingController(text: widget.text);
  }

  @override
  void didUpdateWidget(covariant SyncTextField oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.text != widget.text) {
      _controller.text = widget.text;
    }
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return TextField(
      controller: _controller,
      onChanged: widget.onChanged,
      decoration: widget.decoration,
      onSubmitted: widget.onSubmitted,
    );
  }
}
