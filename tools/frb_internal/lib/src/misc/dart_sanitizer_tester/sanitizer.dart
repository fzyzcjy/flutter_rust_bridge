import 'package:flutter_rust_bridge_internal/src/makefile_dart/test.dart';

extension SanitizerMetadata on Sanitizer {
  String get rustflagValue {
    return switch (this) {
      Sanitizer.asan => 'address',
      Sanitizer.msan => 'memory',
      Sanitizer.lsan => 'leak',
      Sanitizer.tsan => 'thread',
    };
  }

  String get dartSdkBuildOutDir {
    return switch (this) {
      Sanitizer.asan => 'ReleaseASANX64',
      Sanitizer.msan => 'ReleaseMSANX64',
      Sanitizer.lsan => 'ReleaseLSANX64',
      Sanitizer.tsan => 'ReleaseTSANX64',
    };
  }
}
