import 'package:flutter_rust_bridge/src/dart_opaque/_common.dart';
import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/generalized_frb_rust_binding.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
PlatformPointer encodeDartOpaque(Object raw, NativePortType dartHandlerPort,
        GeneralizedFrbRustBinding generalizedFrbRustBinding) =>
    encodeDartOpaqueCommon(raw, dartHandlerPort, generalizedFrbRustBinding);
