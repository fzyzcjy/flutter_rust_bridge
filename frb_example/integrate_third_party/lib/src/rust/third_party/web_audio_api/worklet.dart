// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.37.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'node.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These functions are ignored because they are not marked as `pub`: `load`, `new`
// These functions are ignored because they have generic arguments: `new`, `process`
// These types are ignored because they are not used by any `pub` functions: `AudioParamValues`, `AudioWorkletNodeOptions`, `AudioWorkletRenderer`, `Processor`
// These functions are ignored: `port`

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<AudioWorkletNode>>
abstract class AudioWorkletNode implements AudioNode {
  Future<void> channelConfig();

  /// Represents an integer used to determine how many channels are used when up-mixing and
  /// down-mixing connections to any inputs to the node.
  Future<BigInt> channelCount();

  /// Represents an enumerated value describing the way channels must be matched between the
  /// node's inputs and outputs.
  Future<ChannelCountMode> channelCountMode();

  /// Represents an enumerated value describing the meaning of the channels. This interpretation
  /// will define how audio up-mixing and down-mixing will happen.
  Future<ChannelInterpretation> channelInterpretation();

  /// Unset the callback to run when an unhandled exception occurs in the audio processor.
  Future<void> clearOnprocessorerror();

  /// The [`BaseAudioContext`](crate::context::BaseAudioContext) concrete type which owns this
  /// AudioNode.
  Future<void> context();

  /// Disconnects all outgoing connections from the AudioNode.
  Future<void> disconnect();

  /// Disconnects all outgoing connections at the given output port from the AudioNode.
  ///
  /// # Panics
  ///
  /// This function will panic when
  /// - if the output port is out of bounds for this node
  Future<void> disconnectOutput({required BigInt output});

  Future<BigInt> numberOfInputs();

  Future<BigInt> numberOfOutputs();

  /// Collection of AudioParam objects with associated names of this node
  ///
  /// This map is populated from a list of [`AudioParamDescriptor`]s in the
  /// [`AudioWorkletProcessor`] class constructor at the instantiation.
  Future<void> parameters();

  Future<void> registration();

  /// Update the `channel_count` attribute
  Future<void> setChannelCount({required BigInt v});

  /// Update the `channel_count_mode` attribute
  Future<void> setChannelCountMode({required ChannelCountMode v});

  /// Update the `channel_interpretation` attribute
  Future<void> setChannelInterpretation({required ChannelInterpretation v});

  void dispose();

  bool get isDisposed;
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<Self :: ProcessorOptions>>
abstract class SelfProcessorOptions {
  void dispose();

  bool get isDisposed;
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<dyn Any>>
abstract class Any {
  void dispose();

  bool get isDisposed;
}

abstract class AudioWorkletProcessor {
  /// Handle incoming messages from the linked AudioNode
  ///
  /// By overriding this method you can add a handler for messages sent from the control thread
  /// via the AudioWorkletNode MessagePort.
  ///
  /// Receivers are supposed to consume the content of `msg`. The content of `msg` might
  /// also be replaced by cruft that needs to be deallocated outside of the render thread
  /// afterwards, e.g. when replacing an internal buffer.
  ///
  /// This method is just a shim of the full
  /// [`MessagePort`](https://webaudio.github.io/web-audio-api/#dom-audioworkletprocessor-port)
  /// `onmessage` functionality of the AudioWorkletProcessor.
  Future<void> onmessage({required Any msg});
}