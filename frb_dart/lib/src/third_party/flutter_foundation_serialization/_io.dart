import 'dart:typed_data';

import 'package:flutter_rust_bridge/src/platform_types/_io.dart';
import 'package:flutter_rust_bridge/src/third_party/flutter_foundation_serialization/read_buffer.dart';
import 'package:flutter_rust_bridge/src/third_party/flutter_foundation_serialization/write_buffer.dart';

/// Extensions for ReadBuffer
extension ExtReadBuffer on ReadBuffer {
  /// Reads an Int64 from the buffer.
  PlatformInt64 getPlatformInt64({Endian? endian}) => getInt64(endian: endian);
}

/// Extensions for WriteBuffer
extension ExtWriteBuffer on WriteBuffer {
  /// Writes an Int64 to the buffer.
  void putPlatformInt64(PlatformInt64 value, {Endian? endian}) =>
      putInt64(value, endian: endian);
}
