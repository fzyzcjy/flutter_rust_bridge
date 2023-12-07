import 'dart:typed_data';

// use little endian instead of (default) big endian to improve performance #9640
const Endian _kEndian = Endian.little;

// also ref https://github.com/brendan-duncan/archive/blob/main/lib/src/util/input_stream.dart
/// {@macro flutter_rust_bridge.internal}
class BytesReader {
  /// {@macro flutter_rust_bridge.internal}
  final Uint8List bytes;

  /// {@macro flutter_rust_bridge.internal}
  int get index => _index;
  var _index = 0;

  // ref: protobuf
  // https://github.com/fzyzcjy/yplusplus/issues/9598#issuecomment-1602114654
  /// {@macro flutter_rust_bridge.internal}
  final ByteData byteData;

  /// {@macro flutter_rust_bridge.internal}
  BytesReader(this.bytes) : byteData = ByteData.view(bytes.buffer);

  /// {@macro flutter_rust_bridge.internal}
  Uint8List readBytes(int num) {
    final oldIndex = _index;
    _index += num;
    return bytes.buffer.asUint8List(oldIndex, num);
  }

  /// {@macro flutter_rust_bridge.internal}
  int readUint8() => bytes[_index++];

  /// {@macro flutter_rust_bridge.internal}
  int readInt64() {
    final ans = byteData.getInt64(_index, _kEndian);
    _index += 8;
    return ans;
  }

  /// {@macro flutter_rust_bridge.internal}
  double readFloat32() {
    final ans = byteData.getFloat32(_index, _kEndian);
    _index += 4;
    return ans;
  }

  /// {@macro flutter_rust_bridge.internal}
  double readFloat64() {
    final ans = byteData.getFloat64(_index, _kEndian);
    _index += 8;
    return ans;
  }

  /// {@macro flutter_rust_bridge.internal}
  bool get eof => index == bytes.length;
}

// ref Dart [BytesWriter]
/// {@macro flutter_rust_bridge.internal}
class BytesWriter {
  /// Initial size of internal buffer.
  static const int _initSize = 1024;

  /// Reusable empty [Uint8List].
  ///
  /// Safe for reuse because a fixed-length empty list is immutable.
  static final _emptyList = Uint8List(0);

  /// Current count of bytes written to buffer.
  int _length = 0;

  /// Internal buffer accumulating bytes.
  ///
  /// Will grow as necessary
  Uint8List _buffer;
  ByteData _bufferByteDataView;

  /// {@macro flutter_rust_bridge.internal}
  BytesWriter()
      : _buffer = _emptyList,
        _bufferByteDataView = ByteData.view(_emptyList.buffer);

  /// {@macro flutter_rust_bridge.internal}
  @pragma('vm:prefer-inline')
  void writeBytes(List<int> bytes) {
    _write(bytes.length,
        () => _buffer.setRange(_length, _length + bytes.length, bytes));
  }

  /// {@macro flutter_rust_bridge.internal}
  @pragma('vm:prefer-inline')
  void writeUint8(int value) {
    final required = _length + 1;
    _growIfNotEnoughSpace(required);
    _buffer[length] = value;
    _length = required;
  }

  /// {@macro flutter_rust_bridge.internal}
  @pragma('vm:prefer-inline')
  void writeInt64(int value) {
    _write(8, () => _bufferByteDataView.setInt64(_length, value, _kEndian));
  }

  /// {@macro flutter_rust_bridge.internal}
  @pragma('vm:prefer-inline')
  void writeFloat32(double value) {
    _write(4, () => _bufferByteDataView.setFloat32(_length, value, _kEndian));
  }

  /// {@macro flutter_rust_bridge.internal}
  @pragma('vm:prefer-inline')
  void writeFloat64(double value) {
    _write(8, () => _bufferByteDataView.setFloat64(_length, value, _kEndian));
  }

  @pragma('vm:prefer-inline')
  void _write(int writeBytesLength, void Function() act) {
    final required = _length + writeBytesLength;
    _growIfNotEnoughSpace(required);
    act();
    _length = required;
  }

  @pragma('vm:prefer-inline')
  void _growIfNotEnoughSpace(int required) {
    if (_buffer.length < required) {
      _grow(required);
    }
    assert(_buffer.length >= required);
  }

  void _grow(int required) {
    // We will create a list in the range of 2-4 times larger than
    // required.
    int newSize = required * 2;
    if (newSize < _initSize) {
      newSize = _initSize;
    } else {
      newSize = _pow2roundup(newSize);
    }
    var newBuffer = Uint8List(newSize);
    newBuffer.setRange(0, _buffer.length, _buffer);
    _buffer = newBuffer;
    _bufferByteDataView = ByteData.view(_buffer.buffer);
  }

  Uint8List takeBytes() {
    if (_length == 0) return _emptyList;
    var buffer = Uint8List.view(_buffer.buffer, _buffer.offsetInBytes, _length);
    _clear();
    return buffer;
  }

  /// {@macro flutter_rust_bridge.internal}
  int get length => _length;

  /// {@macro flutter_rust_bridge.internal}
  void clear() {
    _clear();
  }

  void _clear() {
    _length = 0;
    _buffer = _emptyList;
  }

  /// Rounds numbers <= 2^32 up to the nearest power of 2.
  static int _pow2roundup(int x) {
    assert(x > 0);
    --x;
    x |= x >> 1;
    x |= x >> 2;
    x |= x >> 4;
    x |= x >> 8;
    x |= x >> 16;
    return x + 1;
  }
}
