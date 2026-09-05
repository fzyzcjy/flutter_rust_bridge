import 'package:flutter_rust_bridge_internal/src/makefile_dart/test.dart';

String? sanitizerRustflagsForTesting(Sanitizer sanitizer) =>
    sanitizer.rustflags;

extension SanitizerMetadata on Sanitizer {
  String? get rustflags {
    final value = rustflagValue;
    if (value == null) return null;

    return '-Zsanitizer=$value';
  }

  String? get rustflagValue {
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
