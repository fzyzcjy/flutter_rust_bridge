import 'package:flutter_rust_bridge_internal/src/makefile_dart/test.dart';

String? sanitizerRustflagsForTesting(Sanitizer sanitizer) =>
    sanitizer.rustflags;

bool sanitizerUsesRuntimeShutdownForTesting(Sanitizer sanitizer) =>
    sanitizer.usesRuntimeShutdown;

extension SanitizerMetadata on Sanitizer {
  String? get rustflags {
    final value = rustflagValue;
    if (value == null) return null;

    final sanitizerFlag = '-Zsanitizer=$value';
    if (!usesRuntimeShutdown) return sanitizerFlag;

    return '$sanitizerFlag --cfg frb_sanitize_runtime_shutdown';
  }

  bool get usesRuntimeShutdown =>
      this == Sanitizer.asan || this == Sanitizer.lsan;

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
