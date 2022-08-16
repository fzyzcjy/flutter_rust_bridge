export 'dart:typed_data';
import 'dart:typed_data';

/// Non-standard entry point for compatibility with JS.
Int64List int64ListFrom(dynamic raw) => raw as Int64List;

/// Non-standard entry point for compatibility with JS.
Uint64List uint64ListFrom(dynamic raw) => raw as Uint64List;

extension Int64ListExt on Int64List {
  Int64List get inner => this;
}

extension Uint64ListExt on Uint64List {
  Uint64List get inner => this;
}
