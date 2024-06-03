/// Copied and modified from https://github.com/flutter/flutter/blob/master/packages/flutter/lib/src/foundation/serialization.dart
library;

import 'dart:math' as math;
import 'dart:typed_data';

import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/generalized_typed_data/generalized_typed_data.dart';
import 'package:flutter_rust_bridge/src/generalized_uint8list/generalized_uint8list.dart';

// NOTE MAIN MODIFICATION:
// * Uint8List -> BaseGeneralizedUint8List
/// Write-only buffer for incrementally building a [ByteData] instance.
///
/// A WriteBuffer instance can be used only once. Attempts to reuse will result
/// in [StateError]s being thrown.
///
/// The byte order used is [Endian.host] throughout.
class WriteBuffer {
  /// Creates an interface for incrementally building a [ByteData] instance.
  /// [startCapacity] determines the start size of the [WriteBuffer] in bytes.
  /// The closer that value is to the real size used, the better the
  /// performance.
  factory WriteBuffer(
      {int startCapacity = 8, required GeneralizedFrbRustBinding binding}) {
    assert(startCapacity > 0);
    final ByteData eightBytes = ByteData(8);
    final Uint8List eightBytesAsList = eightBytes.buffer.asUint8List();
    return WriteBuffer._(PlatformGeneralizedUint8List(startCapacity, binding),
        eightBytes, eightBytesAsList);
  }

  WriteBuffer._(this._buffer, this._eightBytes, this._eightBytesAsList);

  final BaseGeneralizedUint8List _buffer;
  int _currentSize = 0;
  bool _isDone = false;
  final ByteData _eightBytes;
  final Uint8List _eightBytesAsList;

  // static final Uint8List _zeroBuffer = Uint8List(8);

  void _add(int byte) {
    if (_currentSize == _buffer.length) {
      _resize();
    }
    _buffer[_currentSize] = byte;
    _currentSize += 1;
  }

  void _append(Uint8List other) {
    final int newSize = _currentSize + other.length;
    if (newSize >= _buffer.length) {
      _resize(newSize);
    }
    _buffer.setRange(_currentSize, newSize, other);
    _currentSize += other.length;
  }

  void _addAll(Uint8List data, [int start = 0, int? end]) {
    final int newEnd = end ?? _eightBytesAsList.length;
    final int newSize = _currentSize + (newEnd - start);
    if (newSize >= _buffer.length) {
      _resize(newSize);
    }
    _buffer.setRange(_currentSize, newSize, data);
    _currentSize = newSize;
  }

  void _resize([int? requiredLength]) {
    final int doubleLength = _buffer.length * 2;
    final int newLength = math.max(requiredLength ?? 0, doubleLength);
    // NOTE MODIFIED Originally it creates a new list, but here we directly resize
    // since Rust's Vec has good resize feature
    _buffer.resize(newLength);
  }

  /// Write a Uint8 into the buffer.
  void putUint8(int byte) {
    assert(!_isDone);
    _add(byte);
  }

  /// Write a Uint16 into the buffer.
  void putUint16(int value, {Endian? endian}) {
    assert(!_isDone);
    _eightBytes.setUint16(0, value, endian ?? Endian.host);
    _addAll(_eightBytesAsList, 0, 2);
  }

  /// Write a Uint32 into the buffer.
  void putUint32(int value, {Endian? endian}) {
    assert(!_isDone);
    _eightBytes.setUint32(0, value, endian ?? Endian.host);
    _addAll(_eightBytesAsList, 0, 4);
  }

  // /// Write a Uint64 into the buffer.
  // void putUint64(int value, {Endian? endian}) {
  //   putBigUint64(BigInt.from(value), endian: endian);
  // }

  /// Write a Uint64 into the buffer.
  void putBigUint64(BigInt value, {Endian? endian}) {
    assert(!_isDone);
    byteDataSetUint64(_eightBytes, 0, value, endian ?? Endian.host);
    _addAll(_eightBytesAsList, 0, 8);
  }

  /// Write an Int8 into the buffer.
  void putInt8(int value) {
    assert(!_isDone);
    _eightBytes.setInt8(0, value);
    _addAll(_eightBytesAsList, 0, 1);
  }

  /// Write an Int16 into the buffer.
  void putInt16(int value, {Endian? endian}) {
    assert(!_isDone);
    _eightBytes.setInt16(0, value, endian ?? Endian.host);
    _addAll(_eightBytesAsList, 0, 2);
  }

  /// Write an Int32 into the buffer.
  void putInt32(int value, {Endian? endian}) {
    assert(!_isDone);
    _eightBytes.setInt32(0, value, endian ?? Endian.host);
    _addAll(_eightBytesAsList, 0, 4);
  }

  /// Write an Int64 into the buffer.
  void putInt64(int value, {Endian? endian}) {
    putBigInt64(BigInt.from(value), endian: endian);
  }

  /// Write an Int64 into the buffer.
  void putBigInt64(BigInt value, {Endian? endian}) {
    assert(!_isDone);
    byteDataSetInt64(_eightBytes, 0, value, endian ?? Endian.host);
    _addAll(_eightBytesAsList, 0, 8);
  }

  // NOTE ADD by mimic the 64bit counterpart
  /// Write an Float32 into the buffer.
  void putFloat32(double value, {Endian? endian}) {
    assert(!_isDone);
    // _alignTo(4);
    _eightBytes.setFloat32(0, value, endian ?? Endian.host);
    _addAll(_eightBytesAsList, 0, 4);
  }

  /// Write an Float64 into the buffer.
  void putFloat64(double value, {Endian? endian}) {
    assert(!_isDone);
    // _alignTo(8);
    _eightBytes.setFloat64(0, value, endian ?? Endian.host);
    _addAll(_eightBytesAsList);
  }

  /// Write all the values from a [Uint8List] into the buffer.
  void putUint8List(Uint8List list) {
    assert(!_isDone);
    _append(list);
  }

  /// Write all the values from an [Uint16List] into the buffer.
  void putUint16List(Uint16List list) {
    assert(!_isDone);
    _append(list.buffer.asUint8List(list.offsetInBytes, 2 * list.length));
  }

  /// Write all the values from an [Uint32List] into the buffer.
  void putUint32List(Uint32List list) {
    assert(!_isDone);
    _append(list.buffer.asUint8List(list.offsetInBytes, 4 * list.length));
  }

  /// Write all the values from an [Uint64List] into the buffer.
  void putUint64List(Uint64List list) {
    assert(!_isDone);
    for (final value in list) {
      putBigUint64(value);
    }
  }

  /// Write all the values from an [Int8List] into the buffer.
  void putInt8List(Int8List list) {
    assert(!_isDone);
    _append(list.buffer.asUint8List(list.offsetInBytes, 1 * list.length));
  }

  /// Write all the values from an [Int16List] into the buffer.
  void putInt16List(Int16List list) {
    assert(!_isDone);
    _append(list.buffer.asUint8List(list.offsetInBytes, 2 * list.length));
  }

  /// Write all the values from an [Int32List] into the buffer.
  void putInt32List(Int32List list) {
    assert(!_isDone);
    // _alignTo(4);
    _append(list.buffer.asUint8List(list.offsetInBytes, 4 * list.length));
  }

  /// Write all the values from an [Int64List] into the buffer.
  void putInt64List(Int64List list) {
    assert(!_isDone);
    // _alignTo(8);
    for (final value in list) {
      putBigInt64(value);
    }
  }

  /// Write all the values from a [Float32List] into the buffer.
  void putFloat32List(Float32List list) {
    assert(!_isDone);
    // _alignTo(4);
    _append(list.buffer.asUint8List(list.offsetInBytes, 4 * list.length));
  }

  /// Write all the values from a [Float64List] into the buffer.
  void putFloat64List(Float64List list) {
    assert(!_isDone);
    // _alignTo(8);
    _append(list.buffer.asUint8List(list.offsetInBytes, 8 * list.length));
  }

  // NOTE MODIFIED try remove this to simplify rust side
  // void _alignTo(int alignment) {
  //   assert(!_isDone);
  //   final int mod = _currentSize % alignment;
  //   if (mod != 0) {
  //     _addAll(_zeroBuffer, 0, alignment - mod);
  //   }
  // }

  /// NOTE: We do not call `resize()` to shrink the buffer (which makes the
  /// API nicer since there is no need to pass around the extra size variable).
  /// This is to avoid overhead.
  ///
  /// Details:
  /// The internal implementation of `resize` (to shrink) contains a
  /// call to the Rust `std::Vec::shrink_to_fit`.
  /// By looking at source, we see it eventually calls `Allocator::shrink`.
  /// The default implementation even do alloc-copy-dealloc which contains
  /// a memory copy. Other implementations may be faster, but still
  /// it is overhead.
  WriteBufferRaw intoRaw() {
    if (_isDone) {
      throw StateError(
          'done() must not be called more than once on the same $runtimeType.');
    }
    _isDone = true;
    final raw = _buffer.intoRaw();
    return (ptr: raw.ptr, rustVecLen: raw.length, dataLen: _currentSize);
  }
}

/// {@macro flutter_rust_bridge.internal}
typedef WriteBufferRaw = ({
  PlatformGeneralizedUint8ListPtr ptr,
  int rustVecLen,
  int dataLen
});
