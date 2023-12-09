/// Copied and modified from https://github.com/flutter/flutter/blob/master/packages/flutter/lib/src/foundation/serialization.dart
library;

import 'dart:typed_data';

import 'package:flutter_rust_bridge/src/generalized_typed_data/generalized_typed_data.dart';

/// Read-only buffer for reading sequentially from a [ByteData] instance.
///
/// The byte order used is [Endian.host] throughout.
class ReadBuffer {
  /// Creates a [ReadBuffer] for reading from the specified [data].
  ReadBuffer(this.data);

  /// The underlying data being read.
  final ByteData data;

  /// The position to read next.
  int _position = 0;

  /// Whether the buffer has data remaining to read.
  bool get hasRemaining => _position < data.lengthInBytes;

  /// Reads a Uint8 from the buffer.
  int getUint8() {
    return data.getUint8(_position++);
  }

  /// Reads a Uint16 from the buffer.
  int getUint16({Endian? endian}) {
    final int value = data.getUint16(_position, endian ?? Endian.host);
    _position += 2;
    return value;
  }

  /// Reads a Uint32 from the buffer.
  int getUint32({Endian? endian}) {
    final int value = data.getUint32(_position, endian ?? Endian.host);
    _position += 4;
    return value;
  }

  /// Reads a Uint64 from the buffer.
  int getUint64({Endian? endian}) {
    final int value = data.getUint64(_position, endian ?? Endian.host);
    _position += 8;
    return value;
  }

  // NOTE ADD some of these functions
  /// Reads an Int8 from the buffer.
  int getInt8({Endian? endian}) {
    return data.getInt8(_position++);
  }

  /// Reads an Int16 from the buffer.
  int getInt16({Endian? endian}) {
    final int value = data.getInt16(_position, endian ?? Endian.host);
    _position += 2;
    return value;
  }

  /// Reads an Int32 from the buffer.
  int getInt32({Endian? endian}) {
    final int value = data.getInt32(_position, endian ?? Endian.host);
    _position += 4;
    return value;
  }

  /// Reads an Int64 from the buffer.
  int getInt64({Endian? endian}) {
    final int value = data.getInt64(_position, endian ?? Endian.host);
    _position += 8;
    return value;
  }

  /// Reads a Float32 from the buffer.
  double getFloat32({Endian? endian}) {
    // _alignTo(4);
    final double value = data.getFloat32(_position, endian ?? Endian.host);
    _position += 4;
    return value;
  }

  /// Reads a Float64 from the buffer.
  double getFloat64({Endian? endian}) {
    // _alignTo(8);
    final double value = data.getFloat64(_position, endian ?? Endian.host);
    _position += 8;
    return value;
  }

  /// Reads the given number of Uint8s from the buffer.
  Uint8List getUint8List(int length) {
    final Uint8List list =
        data.buffer.asUint8List(data.offsetInBytes + _position, length);
    _position += length;
    return list;
  }

  /// Reads the given number of Uint16s from the buffer.
  Uint16List getUint16List(int length) {
    final Uint16List list =
        data.buffer.asUint16List(data.offsetInBytes + _position, length);
    _position += 2 * length;
    return list;
  }

  /// Reads the given number of Uint32s from the buffer.
  Uint32List getUint32List(int length) {
    final Uint32List list =
        data.buffer.asUint32List(data.offsetInBytes + _position, length);
    _position += 4 * length;
    return list;
  }

  /// Reads the given number of Uint64s from the buffer.
  Uint64List getUint64List(int length) {
    final Uint64List list =
        data.buffer.asUint64List(data.offsetInBytes + _position, length);
    _position += 8 * length;
    return list;
  }

  /// Reads the given number of Int8s from the buffer.
  Int8List getInt8List(int length) {
    final Int8List list =
        data.buffer.asInt8List(data.offsetInBytes + _position, length);
    _position += 1 * length;
    return list;
  }

  /// Reads the given number of Int16s from the buffer.
  Int16List getInt16List(int length) {
    final Int16List list =
        data.buffer.asInt16List(data.offsetInBytes + _position, length);
    _position += 2 * length;
    return list;
  }

  /// Reads the given number of Int32s from the buffer.
  Int32List getInt32List(int length) {
    // _alignTo(4);
    final Int32List list =
        data.buffer.asInt32List(data.offsetInBytes + _position, length);
    _position += 4 * length;
    return list;
  }

  /// Reads the given number of Int64s from the buffer.
  Int64List getInt64List(int length) {
    // _alignTo(8);
    final Int64List list =
        data.buffer.asInt64List(data.offsetInBytes + _position, length);
    _position += 8 * length;
    return list;
  }

  /// Reads the given number of Float32s from the buffer
  Float32List getFloat32List(int length) {
    // _alignTo(4);
    final Float32List list =
        data.buffer.asFloat32List(data.offsetInBytes + _position, length);
    _position += 4 * length;
    return list;
  }

  /// Reads the given number of Float64s from the buffer.
  Float64List getFloat64List(int length) {
    // _alignTo(8);
    final Float64List list =
        data.buffer.asFloat64List(data.offsetInBytes + _position, length);
    _position += 8 * length;
    return list;
  }

// NOTE MODIFIED try remove this to simplify rust side
// void _alignTo(int alignment) {
//   final int mod = _position % alignment;
//   if (mod != 0) {
//     _position += alignment - mod;
//   }
// }
}
